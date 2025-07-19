use sqlx::{Database, Pool, Postgres, QueryBuilder};
use types::payload::admin::{EditUser, NewHashedUser, NewUser};
use types::models::init::{User, UserRole};
use crate::queries::QueryErr;





pub async fn insert_one_user(
    user: NewHashedUser,
    db: &Pool<Postgres>
) -> Result<(), QueryErr>{
    
    sqlx::query(
        "INSERT INTO users (
            email, password, salt,
            username, first_name, last_name,
            location, phone_no, role
        )
        VALUES (
            $1, $2, $3, 
            $4, $5, $6, 
            $7, $8, $9
        )"
    )
    .bind(&user.email)
    .bind(&user.password)
    .bind(&user.salt)
    .bind(&user.username)
    .bind(&user.first_name)
    .bind(&user.last_name)
    .bind(&user.location)
    .bind(&user.phone_no)
    .bind(&user.role) // this will work if UserRole implements sqlx::Type
    .execute(db)
    .await
    .map_err(|e| QueryErr::FailedToCollect(format!("{}", e)))?;

    Ok(())
}


pub async fn patch_user(
    id: String,
    user: EditUser,
    db: &Pool<Postgres>
) -> Result<(), QueryErr> {

    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
        "UPDATE users SET"
    );
    let mut empty = true;

    if let Some(email) = user.email{
        query.push("email = ")
            .push_bind(email)
            .push(", ");
        empty = false;
    }

    if let Some(first_name) = user.first_name{
        query.push("first_name = ")
            .push_bind(first_name)
            .push(", ");
        empty = false;
    }

    if let Some(last_name) = user.last_name{
        query.push("last_name = ")
            .push_bind(last_name)
            .push(", ");
        empty = false;
    } 

    if let Some(location) = user.location{
        query.push("location = ")
            .push_bind(location)
            .push(", ");
        empty = false;
    } 


    if let Some(phone_no) = user.phone_no{
        query.push("phone_no = ")
            .push_bind(phone_no)
            .push(", ");
        empty = false;
    } 

    if let Some(role) = user.role{
        query.push("role = ")
            .push_bind(role)
            .push(", ");
        empty = false;
    }


    if empty {
        return Err(QueryErr::EmptyValue);
    }

    query.push("WHERE id = ").push_bind(id);


    query.build()
        .execute(db)
        .await
        .map_err(|_| 
            QueryErr::FailedToPatch(
                "Failed to patch user".to_string()
            )
        )?;



    Ok(())
}



pub async fn get_list_user(
    bounds: (i64, i64),
    db: &Pool<Postgres>
) -> Result<Vec<User>, QueryErr> {
    sqlx::query_as!(
        User,
        "SELECT 
            id,email,password, 
            salt, username, first_name, 
            last_name, location, phone_no,
            role as \"role:UserRole\" 
        FROM users 
        ORDER BY username 
        LIMIT $1 OFFSET $2",
        bounds.0,
        bounds.1
    )
    .fetch_all(db)
    .await
    .map_err(|_|
        QueryErr::FailedToCollect("Failed to fetch user list".to_string())   
    )
}