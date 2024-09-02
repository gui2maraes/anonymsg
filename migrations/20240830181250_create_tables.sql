-- Add migration script here
create table keymap(
    name varchar(100) primary key,
    public_key text not null,
);
create table posts(
    id uuid primary key,
    to varchar(100) references keymap(name),
    content text not null,
    sent_at timestamptz not null
);
