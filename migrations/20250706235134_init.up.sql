-- Add up migration script here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash TEXT NOT NULL,
    secret TEXT NULL,
    CONSTRAINT users_email_key UNIQUE (email)
);

CREATE TABLE "groups" (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    group_id INTEGER NOT NULL,
    title VARCHAR(100) NOT NULL,
    image CHAR(1) NOT NULL,
    CONSTRAINT groups_users_fk FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE cards (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    group_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    username VARCHAR(100) NULL,
    password TEXT NULL,
    website VARCHAR(255) NULL,
    notes TEXT NULL,
    image CHAR(1) NOT NULL,
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT cards_group_id_fkey FOREIGN KEY (group_id) REFERENCES "groups"(id),
    CONSTRAINT cards_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
