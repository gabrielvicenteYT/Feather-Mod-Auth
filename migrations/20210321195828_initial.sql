-- Enable uuid functions in postgres
CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE SCHEMA IF NOT EXISTS minos;

CREATE TABLE minos.users
(
    id            UUID        NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    username      varchar(32) NOT NULL,
    first_name    varchar(32) NOT NULL,
    last_name     varchar(32) NOT NULL,
    password      BYTEA       NOT NULL, --The password is encrypted.
    creation_date timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_date   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    banned        BOOLEAN     NOT NULL DEFAULT FALSE
);
