-- Your SQL goes here

CREATE TABLE tb_user
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(50) NOT NULL,
    password   VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL
)