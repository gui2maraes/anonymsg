-- Add migration script here

create extension "pg_trgm";
-- create extension "fuzzystrmatch";
SET pg_trgm.similarity_threshold = 0.2;

create table keymap(
    name varchar(100) primary key,
    public_key text not null
);
CREATE INDEX names_idx ON keymap USING GIST (name gist_trgm_ops);

create table messages(
    id uuid primary key,
    recipient varchar(100) references keymap(name),
    content text not null,
    sent_at timestamptz not null
);
