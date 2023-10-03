CREATE TABLE board_column_relations (
    board_id VARCHAR NOT NULL,
    column_id VARCHAR UNIQUE NOT NULL,
    PRIMARY KEY (board_id, column_id),
    FOREIGN KEY (board_id) REFERENCES boards(id),
    FOREIGN KEY (column_id) REFERENCES columns(id)
);
