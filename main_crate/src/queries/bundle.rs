use sqlx::{Pool, Postgres, QueryBuilder, Transaction};
use types::{payload::{self, bundle::{ BundleJoinedProduct, CreateBundle}, QueryBounds}};

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

