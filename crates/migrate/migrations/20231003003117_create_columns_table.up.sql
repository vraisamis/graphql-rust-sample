CREATE TABLE columns (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    board_id VARCHAR NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards(id)
);
