CREATE TABLE likes (
   id uuid NOT NULL PRIMARY KEY,
   post_id uuid NOT NULL,
   user_id uuid NOT NULL 
);