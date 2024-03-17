-- Add migration script here
alter table animes
    modify synopsis varchar(4000) not null;