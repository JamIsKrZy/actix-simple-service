-- Add down migration script here




DROP TABLE IF EXISTS bundle_items;
DROP TABLE IF EXISTS bundles;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS carts;

DROP TYPE purchase_type;
DROP TYPE product_status;
