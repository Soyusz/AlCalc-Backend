CREATE TABLE entries (
    id uuid NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    price FLOAT NOT NULL,
    voltage FLOAT NOT NULL,
    volume FLOAT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT 'f',
    photo VARCHAR NOT NULL
);