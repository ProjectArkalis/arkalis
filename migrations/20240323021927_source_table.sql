-- Add migration script here
create table sources (
    id int unsigned auto_increment primary key,
    name varchar(255) not null,
    source_type int unsigned not null,
    priority tinyint not null
);

create unique index name_source_type_index on sources(name, source_type);