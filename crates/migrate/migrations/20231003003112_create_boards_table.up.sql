CREATE TABLE boards (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    owner_id VARCHAR,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);
