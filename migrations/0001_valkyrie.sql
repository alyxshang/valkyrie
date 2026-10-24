-- VALKYRIE by Alyx Shang.
-- Licensed under the FSL v1.

-- A table to
-- store information
-- about the current
-- Valkyrie instance
-- in the database.
create table instance (
    host varchar not null,
    name varchar not null primary key,
    admin varchar not null,
    description text not null
);

-- A table to model
-- information about a 
-- user in the database.
create table users (
    username varchar not null primary key,
    password varchar not null,
    display_name text not null,
    description text not null,
    is_admin boolean not null
);

-- A structure to model
-- information about a 
-- user's API token
-- in the database.
create table api_tokens (
    owner varchar not null,
    token varchar not null primary key
);

-- A structure to model
-- information about a post
-- a user has made in the
-- database.
create table posts (
    post_id varchar not null primary key,
    created_at varchar not null,
    created_by varchar not null, 
    post_content text not null
);
