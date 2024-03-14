-- Add migration script here
create table animes (
    id int unsigned auto_increment primary key,
    titles json not null,
    title_search varchar(4000) not null,
    synopsis varchar(255) not null,
    thumbnail_id varchar(255),
    banner_id varchar(255),
    is_hidden bool not null,
    is_nsfw bool not null,
    created_by varchar(36) not null,
    created_at timestamp not null,
    genre bigint unsigned not null,
    release_date date not null,
    anime_in_lists json not null
);