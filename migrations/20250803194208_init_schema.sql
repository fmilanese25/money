create table expenses (
  id serial primary key,
  date date not null,
  amount integer not null,
  category varchar(63) not null,
  message varchar(2047),
  image_url image_url varchar(255),
  longitude double precision,
  latitude double precision
);