-- Add migration script here
create table episodes (
    id varchar(32) not null primary key ,
    name varchar(36) not null,
    cover_id varchar(255),
    season_id int unsigned not null ,
    source_id int unsigned not null ,
    lbry_media_id varchar(255),
    file_name varchar(255),
    is_nsfw bool,
    sequence smallint not null ,
    foreign key (season_id) references seasons(id),
    foreign key (source_id) references sources(id)
);

create unique index episodes_anime_source_sequence_idx on episodes(season_id, source_id, sequence);