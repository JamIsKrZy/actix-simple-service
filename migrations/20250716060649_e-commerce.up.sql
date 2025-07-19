-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE purchase_type AS ENUM ('product', 'bundle');

CREATE TYPE product_status AS ENUM ('available', 'unavailable');

CREATE TABLE IF NOT EXISTS carts (
    user_id UUID NOT NULL,
    item_type purchase_type NOT NULL,
    item_id INTEGER NOT NULL,
    quantity SMALLINT NOT NULL,
    time_created TIMESTAMP NOT NULL,

    PRIMARY KEY (user_id, item_type, item_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS products (
    id SERIAL UNIQUE,
    name VARCHAR(128) NOT NULL UNIQUE,
    description VARCHAR(256),
    price NUMERIC(12,2) NOT NULL,
    stocks BIGINT NOT NULL,

    created_by UUID NOT NULL,
    edited_by UUID DEFAULT NULL,
    edited_time TIMESTAMP DEFAULT NULL,
    status product_status DEFAULT 'unavailable',

    PRIMARY KEY (id, name),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (edited_by) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS bundles (
    id SERIAL UNIQUE,
    name VARCHAR(128) NOT NULL,
    price NUMERIC(12,2) NOT NULL,

    status product_status DEFAULT 'unavailable',
    created_by UUID NOT NULL,
    edited_by UUID NOT NULL,
    edited_time TIMESTAMP NOT NULL,

    PRIMARY KEY (id, name),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (edited_by) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS bundle_items (
    bundle_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,

    PRIMARY KEY (bundle_id, product_id),
    FOREIGN KEY (bundle_id) REFERENCES bundles(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);
