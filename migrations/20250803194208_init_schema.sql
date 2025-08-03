create table users (
    id serial primary key,
    name text not null
);

create table expenses (
    id serial primary key,
    user_id integer not null references users(id) on delete cascade,
    date date not null,
    amount bigint not null,
    category text not null,
    message text,
    image_url text
);