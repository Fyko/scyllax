create keyspace scyllax with replication = {
	'class': 'SimpleStrategy',
	'replication_factor': 1
};

use scyllax;

create table person (
	id timeuuid primary key,
	email text
	-- camel case for the sake of backwards compat
	createdAt timestamp
);

insert into person(id, email, "createdAt") values (e01e84d6-414c-11ee-be56-0242ac120002, 'foo1@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e880a-414c-11ee-be56-0242ac120002, 'foo2@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e89c2-414c-11ee-be56-0242ac120002, 'foo3@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e8b2a-414c-11ee-be56-0242ac120002, 'foo4@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e8db4-414c-11ee-be56-0242ac120002, 'foo5@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e911a-414c-11ee-be56-0242ac120002, 'foo6@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e9354-414c-11ee-be56-0242ac120002, 'foo7@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e94e4-414c-11ee-be56-0242ac120002, 'foo8@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e9854-414c-11ee-be56-0242ac120002, 'foo9@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e99ee-414c-11ee-be56-0242ac120002, 'foo10@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e9b60-414c-11ee-be56-0242ac120002, 'foo11@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e9cc8-414c-11ee-be56-0242ac120002, 'foo12@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01e9e3a-414c-11ee-be56-0242ac120002, 'foo13@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01ea1d2-414c-11ee-be56-0242ac120002, 'foo14@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01ea380-414c-11ee-be56-0242ac120002, 'foo15@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01ea4f2-414c-11ee-be56-0242ac120002, 'foo16@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01ea682-414c-11ee-be56-0242ac120002, 'foo17@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01ea81c-414c-11ee-be56-0242ac120002, 'foo18@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01eab14-414c-11ee-be56-0242ac120002, 'foo19@scyllax.local', toUnixTimestamp(now()));
insert into person(id, email, "createdAt") values (e01eaca4-414c-11ee-be56-0242ac120002, 'foo20@scyllax.local', toUnixTimestamp(now()));
