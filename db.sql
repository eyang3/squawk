CREATE TABLE users (
  id serial primary key,
  email text unique,
  userid text unique,
  username text,
  name text,
  avatar_src text,
  password varchar(128),
  salt varchar(32),
  isgoogle boolean,
  googleid text, 
  ip varchar(16)
);

