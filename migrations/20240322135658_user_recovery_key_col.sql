-- Add migration script here
alter table users add column recovery_key varchar(36);