-- Add migration script here
create table users
(
    id          uuid primary key,
    username    text        not null,
    password    text        not null,
    create_time timestamptz not null default now(),
    unique (username)
);

create table polls
(
    id          uuid primary key,
    title       text        not null,
    creator     uuid        not null references users (id),
    create_time timestamptz not null default now()
);

create table poll_options
(
    id      uuid unique,
    poll_id uuid references polls (id),
    text    text not null,
    primary key (id, poll_id)
);

create view polls_view as
select polls.id          as poll_id,
       title             as poll_title,
       polls.create_time as poll_create_time,
       users.id          as user_id,
       users.username    as user_username,
       users.password    as user_password,
       users.create_time as user_create_time
from polls
         left join users on users.id = polls.creator;

create table votes
(
    voter     uuid not null references users (id),
    poll      uuid not null references polls (id),
    option    uuid not null references poll_options (id),
    vote_time timestamptz not null default now(),
    unique (voter, poll)
);
