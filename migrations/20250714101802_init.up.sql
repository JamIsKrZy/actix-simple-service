-- Add up migration script here


CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TYPE user_role AS ENUM ('admin', 'regular', 'employee');

CREATE TABLE 
    IF NOT EXISTS users(
        id UUID UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
        email VARCHAR(64) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        salt VARCHAR(128) NOT NULL,
        username VARCHAR(16) UNIQUE NOT NULL,
        first_name VARCHAR(32) NOT NULL,
        last_name VARCHAR(32) NOT NULL,
        location VARCHAR(128),
        phone_no VARCHAR(20) NOT NULL,
        role user_role NOT NULL DEFAULT 'employee'
    );


