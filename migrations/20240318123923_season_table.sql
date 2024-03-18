-- Add migration script here
create table seasons (
    id int unsigned auto_increment primary key,
    name varchar(255) not null,
    cover_id varchar(255),
    anime_id int unsigned not null,
    sequence smallint unsigned not null,
    foreign key (anime_id) references animes(id)
);