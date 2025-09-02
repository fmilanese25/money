create table expenses (
    id serial primary key,
    date date not null,
    amount integer not null,
    category text not null,
    message text,
    image_url text
);