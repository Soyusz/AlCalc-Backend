CREATE TABLE posts (
    id uuid NOT NULL PRIMARY KEY,
    user_id uuid NOT NULL,
    location VARCHAR,
    title VARCHAR NOT NULL,
    photos text[] NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY(user_id) REFERENCES users(id)
);
