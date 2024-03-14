-- Add migration script here
create table users (
    id varchar(36) not null primary key,
    display_name varchar(255) not null,
    role tinyint unsigned not null
);

alter table animes add constraint foreign key (created_by) references users(id);