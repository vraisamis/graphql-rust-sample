CREATE TABLE cards (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    column_id VARCHAR NOT NULL,
    FOREIGN KEY (column_id) REFERENCES columns(id)
);
