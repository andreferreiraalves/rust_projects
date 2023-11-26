CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name Varchar(255) not null,
    email Varchar(255) not null,
    password varchar(255) not null
);