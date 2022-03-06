CREATE TABLE users (
    id uuid NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT 'f'
);