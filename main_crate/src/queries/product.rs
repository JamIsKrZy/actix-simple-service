use sqlx::{Pool, Postgres, QueryBuilder};
use types::{models::e_commerce::Product, payload::product::{ProductJoinedUser, RecordedEditProduct, TheProduct}};

use crate::queries::QueryErr;



type QueryReturn = Result<(), QueryErr>; 



pub async fn insert_one_product(
    product: TheProduct<'_>,
    db: &Pool<Postgres>
) -> QueryReturn {
    
    sqlx::query(
        "INSERT INTO products (
            name, description, price, stocks, created_by
        )
        VALUES (
            $1, $2, $3, $4, $5
        )"
    )
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.stocks)
    .bind(product.created_by)
    .execute(db)
    .await
    .map_err(|_| 
        QueryErr::FailedToInsert("Failed to insert product!".to_string())
    )?;
    
    Ok(())
}




pub async fn patch_product(
    product_id: i32,
    edited_product: RecordedEditProduct,
    db: &Pool<Postgres>
) -> QueryReturn {


    let mut empty = true; 
    let mut start_parenthesis = false;
    let mut query = QueryBuilder::new(
        "UPDATE products SET "
    );


    if let Some(description) = edited_product.description {
        query.push(" description = ")
            .push_bind(description);
        empty = false;    
        start_parenthesis = true;
    }

    if let Some(name) = edited_product.name {
        if start_parenthesis {
            query.push(", ");
        }

        query.push("name = ")
            .push_bind(name);
        empty = false;    
        start_parenthesis = true;
    }      

    if let Some(price) = edited_product.price {
        if start_parenthesis {
            query.push(", ");
        }

        query.push("price = ")
            .push_bind(price);
        empty = false;    
        start_parenthesis = true;
    }   

    if let Some(stocks) = edited_product.stocks {
        if start_parenthesis {
            query.push(", ");
        }

        query.push("stocks = ")
            .push_bind(stocks);
        empty = false;    
        start_parenthesis = true;
    }

    if let Some(status) = edited_product.status {
        if start_parenthesis {
            query.push(", ");
        }

        query.push("status = ")
            .push_bind(status);
        empty = false;    
    }

    if empty {
        return Err(QueryErr::EmptyItem);
    }

    query.push(" WHERE id = ").push_bind(product_id);


    query.build()
        .execute(db)
        .await
        .map_err(|e| 
            QueryErr::FailedToPatch(format!("Failed to patch product!. Err: {}", e ))
        )?;

    Ok(())
}



pub async fn product_list(
    bound: (i64, i64),
    db: &Pool<Postgres>
) -> Result<Vec<ProductJoinedUser>, QueryErr> {

    let list: Vec<ProductJoinedUser> = sqlx::query_as(
        "
        SELECT 
            p.id, p.name, p.description,
            p.price, p.stocks, p.edited_time, 
            p.status, 
            u.id AS created_by_id, 
            u.username AS created_by_name,
            e.id AS edited_by_id, 
            e.username AS edited_by_name
        FROM products p
        LEFT JOIN users u ON p.created_by = u.id
        LEFT JOIN users e ON p.edited_by = e.id
        ORDER BY p.id
        LIMIT $1 OFFSET $2"
    )
    .bind(bound.0)
    .bind(bound.1)
    .fetch_all(db)
    .await
    .map_err(|e|{
        // mapp err did not handle if query fails or fail fetch
        QueryErr::FailedToCollect(format!("{}", e))
    })?;
    
    Ok(list)
}   