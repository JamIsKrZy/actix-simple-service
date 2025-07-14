-- Active: 1752462507054@@127.0.0.1@5432@actix_service



CREATE EXTENSION IF NOT EXISTS "uuid-ossp";



CREATE TYPE PRODUCT_STATUS AS ENUM ('Availble', 'Unavailable');
CREATE TYPE PURCHASE_TYPE AS ENUM ('Product', 'Bundle');
CREATE TYPE PURCHASE_STATUS AS ENUM ('Preparing', 'Ready', 'Collected');
CREATE TYPE USER_ROLE AS ENUM ('Admin', 'Regular', 'Staff');




CREATE Table users (
    user_id UUID UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(64) UNIQUE NOT NULL,
    password VARCHAR(255) not NULL,
    salt VARCHAR(64) NOT NULL,
    username VARCHAR(16) UNIQUE NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32) NOT NULL,
    location VARCHAR(128),
    phone_no VARCHAR(20),
    role USER_ROLE NOT NULL DEFAULT 'Regular'
);

drop table users;



CREATE TABLE purchased(
    user_id UUID NOT NULL,
    item_type PURCHASE_TYPE NOT NULL,
    item_id INTEGER NOT NULL,
    quantity SMALLINT NOT NULL,
    time_created TIMESTAMP NOT NULL,
    status PURCHASE_STATUS DEFAULT 'Preparing',

    PRIMARY KEY (user_id, item_type),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
)


create Table carts(
    user_id UUID NOT NULL,
    item_type PURCHASE_TYPE NOT NULL,
    item_id INTEGER NOT NULL,
    quantity SMALLINT NOT NULL,
    time_created TIMESTAMP NOT NULL,

    PRIMARY KEY (user_id, item_type),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);


CREATE Table products (
    product_id SERIAL PRIMARY KEY,
    product_name VARCHAR(128) NOT NULL,
    description VARCHAR(256),
    price NUMERIC(12,2) NOT NULL,
    Stocks INTEGER DEFAULT 0,
    created_by UUID NOT NULL,
    edited_by UUID NOT NULL,
    edited_time TIMESTAMP NOT NULL,
    status PRODUCT_STATUS NOT NULL DEFAULT 'Unavailable',


    FOREIGN KEY (created_by) REFERENCES users(user_id),
    FOREIGN KEY (edited_by) REFERENCES users(user_id)
);

drop table products;


CREATE Table bundles (
    bundle_id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    price NUMERIC(12,2) NOT NULL,
    status PRODUCT_STATUS DEFAULT 'Unavailable',
    created_by UUID NOT NULL,
    edited_by UUID NOT NULL,

    FOREIGN KEY (created_by) REFERENCES users(user_id),
    FOREIGN KEY (edited_by) REFERENCES users(user_id)
);

drop Table bundles;


CREATE TABLE bundle_item(
    bundle_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL, 
    quantity INTEGER NOT NULL,

    PRIMARY KEY (bundle_id, product_id),
    FOREIGN KEY (bundle_id) REFERENCES bundles(bundle_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id)
)






