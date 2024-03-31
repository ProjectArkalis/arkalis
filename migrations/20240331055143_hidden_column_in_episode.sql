-- Add migration script here
alter table episodes add column is_hidden bool not null default false;