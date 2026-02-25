-- Add up migration script here
create table if not exists user_account
(
    id        int primary key generated always as identity,
    user_name varchar(255) not null,
    created_timestamp timestamp not null default now(),
    updated_timestamp timestamp not null default now()
);
