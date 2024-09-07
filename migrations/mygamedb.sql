begin;

create table items (
    "id" serial primary key,
    "name" varchar(64) not null,
    "category" varchar(64) not null,
    "created_at" timestamp not null,
    "updated_at" timestamp not null
);

commit;