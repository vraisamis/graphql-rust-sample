CREATE TABLE user_board_relations (
    user_id VARCHAR NOT NULL,
    board_id VARCHAR UNIQUE NOT NULL,
    PRIMARY KEY (user_id, board_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (board_id) REFERENCES boards(id)
);
