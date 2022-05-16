CREATE TABLE sessions (
    id uuid NOT NULL PRIMARY KEY,
    user_id uuid NOT NULL,
    authorized BOOLEAN NOT NULL DEFAULT 'f',
    expiration TIMESTAMP NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY(user_id) REFERENCES users(id)
);
