CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE users (
    user_name VARCHAR(100) PRIMARY KEY,
    correct_answers INTEGER NOT NULL,
    incorrect_answers INTEGER NOT NULL
);

CREATE TABLE answer_type (
    answer_type VARCHAR(20) PRIMARY KEY
);

CREATE TABLE answers (
    answer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_name VARCHAR(100),
    answer_type VARCHAR(20),
    FOREIGN KEY (user_name) REFERENCES users(user_name),
    FOREIGN KEY (answer_type) REFERENCES answer_type(answer_type)
);

