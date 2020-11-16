-- This sql is reference right now. Diesel will handle it through migrations.


CREATE DATABASE IF NOT EXISTS mutezone;

CREATE TABLE User{
    id SERIAL PRIMARY KEY NOT NULL,
    username CHARACTER VARYING(200) NOT NULL UNIQUE,
    password CHARACTER VARYING(200) NOT NULL,
    email CHARACTER VARYING(200) NOT NULL UNIQUE,
    description CHARACTER VARYING(2000),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    last_login TIMESTAMP,
    admin BOOL NOT NULL,
}

CREATE TABLE Track{
    id ID SERIAL PRIMARY KEY NOT NULL,
    title CHARACTER VARYING(200) NOT NULL,
    duration INTEGER NOT NULL,
    description CHARACTER VARYING(2000),
    credits CHARACTER VARYING(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
}