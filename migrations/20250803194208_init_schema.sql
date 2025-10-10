create table expenses (
  id serial primary key,
  date timestamp not null,
  amount integer not null,
  category varchar(63) not null,
  image_url varchar(255),
  longitude double precision,
  latitude double precision,
  message varchar(2047)
);