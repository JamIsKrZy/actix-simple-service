use sqlx::{Acquire, Pool, Postgres, QueryBuilder, Transaction};
use types::payload::{self, bundle::{ BundleJoinedProduct, CreateBundle, EditBundle}, QueryBounds};

use crate::queries::QueryErr;




type QueryResult = Result<(), QueryErr>; 

pub async fn insert_one_bundle(
    bundle: CreateBundle,
    db: &Pool<Postgres>
) -> QueryResult {

    let mut tx = db.begin().await
        .map_err(|_| {
            QueryErr::FailedToBeginTransaction
        })?;


    if bundle.created_by.is_none() {
        return Err(QueryErr::ItemNoHistoryRecord);
    }


    // create bundle row
    let item_key: (i32, ) = sqlx::query_as(
        "INSERT INTO bundles (
            name, price, status,
            created_by
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING id"
    )
    .bind(bundle.name)
    .bind(bundle.price)
    .bind(bundle.status)
    .bind(bundle.created_by)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        QueryErr::FailedToInsert(format!("{}", e))
    })?;


    //create bundle item row
    insert_item_bundles(item_key.0, bundle.items, &mut tx).await?;

    tx.commit()
    .await
    .map_err(|_| QueryErr::FailedCommitTransaction)
}



async fn insert_item_bundles(
    bundle_fk: i32,
    items: Vec<payload::bundle::BundleItem>,
    tx: &mut Transaction<'static, Postgres>
) -> QueryResult{

    let mut query =  QueryBuilder::new(
        "INSERT INTO bundle_items ( 
            bundle_id, product_id, quantity
        )"
    );

    query.push_values(items, |mut a, i| {
        a.push_bind(bundle_fk)
        .push_bind(i.product_id)
        .push_bind(i.quantity);
    });
    
    query.build()
        .execute(&mut **tx)
        .await
        .map_err(|e|
            QueryErr::FailedToInsert(e.to_string())  
        )?;
    
    Ok(())
}



pub async fn patch_bundle(
    id: i32,
    to_edit: EditBundle,
    db: &Pool<Postgres>
) -> QueryResult {

    let mut tx = db.begin().await
        .map_err(|_| QueryErr::FailedToBeginTransaction)?;
    let mut happen = false;

    if to_edit.bundle_has_some() {
        patch_bundle_table(id, &to_edit, &mut tx).await?;
        happen = true
    } 

    if to_edit.bundle_item_has_some() {
        patch_bundle_item_table(id, &to_edit, &mut tx).await?;
        happen = true
    }

    if happen {
        tx.commit()
            .await
            .map_err(|_| {
                QueryErr::FailedCommitTransaction
            })?;
    }
    

    Ok(())
}   


macro_rules! pack_query_field {
    ($query_builder:expr, $from:expr, $field:ident, $start_coma:ident) => {
        if let Some(ref value) = $from.$field {
            if $start_coma {
                $query_builder.push(", ");
            }
            $query_builder
                .push(concat!(stringify!($field), " = "))
                .push_bind(value);

            $start_coma = true;
        }
    };
}


async fn patch_bundle_table(
    id: i32,
    to_edit: &EditBundle,
    tx: &mut Transaction<'_, Postgres>
) -> QueryResult {

    let mut query = QueryBuilder::new(
        "UPDATE bundles SET "
    );

    let mut start_coma = false;

    pack_query_field!(query, to_edit, name, start_coma);
    pack_query_field!(query, to_edit, description, start_coma);
    pack_query_field!(query, to_edit, price, start_coma);
    pack_query_field!(query, to_edit, stocks, start_coma);

    query.push(" WHERE id = ")
        .push_bind(id);

    query.build()
        .execute(&mut **tx)
        .await
        .map_err(|e| 
            QueryErr::FailedToPatch(e.to_string())
        )?;


    Ok(())
}


async fn patch_bundle_item_table(
    bundle_id: i32,
    to_edit: &EditBundle,
    tx: &mut Transaction<'_, Postgres>
) -> QueryResult {

    // delete bundle item 
    if let Some(ref list) = to_edit.remove_product{
        for &id in list {
            sqlx::query(
            "DELETE FROM bundle_items 
                WHERE product_id = $1 AND bundle_id = $2"
            )
            .bind(id)
            .bind(bundle_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| 
                QueryErr::FailedToDelete(format!("{}: {}", line!(), e))
            )?;
        }
    } 

    if let Some(ref list) = to_edit.add_product{
        let mut q = QueryBuilder::new(
        "INSERT INTO bundle_items (
                bundle_id, product_id, quantity
            )"
        );

        q.push_values(list, | mut q, i|{
            q.push_bind(bundle_id)
                .push_bind(i.product_id)
                .push_bind(i.quantity);
        });

        q.build()
            .execute(&mut **tx)
            .await
            .map_err(|e| QueryErr::FailedToInsert(e.to_string()))?;
    }


    if let Some(ref list) = to_edit.edit_product {
        let mut q = QueryBuilder::new(
        "UPDATE products AS p
            SET quantity = v.quantity
            FROM ( "
        );

        q.push_values(list, |mut q, i|{
            q.push_bind(bundle_id)
                .push_bind(i.product_id)
                .push_bind(i.quantity);
        });

        q.push(
            ") AS v(bundle_id, product_id, quantity)
            WHERE p.bundle_id = v.bundle_id 
            AND p.product_id = v.product_id"
        );

        q.build()
            .execute(&mut **tx)
            .await
            .map_err(|e|
                QueryErr::FailedToPatch(e.to_string())
            )?;

    }

    Ok(())
}




pub async fn bundle_list(
    bounds: QueryBounds,
    db: &Pool<Postgres>
) -> Result<Vec<BundleJoinedProduct>, QueryErr>{

    let (limit, offset) = bounds.finalize();

    let mut bundle_list: Vec<BundleJoinedProduct> = sqlx::query_as(
        "SELECT 
            id, name, price, status
        FROM bundles
        ORDER BY created_at
        LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .map_err(|e|{
        QueryErr::FailedToCollect(e.to_string())
    })?;

    for bundle in bundle_list.iter_mut(){

        let Ok(items) = sqlx::query_as(
            "SELECT 
                b.quantity,
                b.product_id,
                p.name AS product_name
            FROM bundle_items b
            LEFT JOIN products p ON b.product_id = p.id
            WHERE bundle_id = $1"
        )
        .bind(bundle.id)
        .fetch_all(db)
        .await
        else { continue; };

        bundle.join_product(items);
    }    

    Ok(bundle_list)
}

