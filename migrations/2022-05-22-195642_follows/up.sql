CREATE TABLE follows (
    id uuid NOT NULL PRIMARY KEY,
    follower_id uuid NOT NULL,
    followed_id uuid NOT NULL,
    CONSTRAINT fk_follows FOREIGN KEY(follower_id) REFERENCES users(id),
    CONSTRAINT fk_followed FOREIGN KEY(followed_id) REFERENCES users(id)
)
