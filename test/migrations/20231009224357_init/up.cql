create keyspace if not exists scyllax with replication = {
	'class': 'SimpleStrategy',
	'replication_factor': 1
};

create table if not exists scyllax.person (
	id timeuuid primary key,
	email text,
	age int,
	data text, -- json type
	kind int,
	-- camel case for the sake of backwards compat
	"createdAt" timestamp
);

create materialized view if not exists scyllax.person_by_email as
  select *
  from person
  where email is not null and id is not null
  primary key (email, id);

insert into scyllax.person(id, email, age, data, kind, "createdAt") values (e01e84d6-414c-11ee-be56-0242ac120002, 'foo1@scyllax.local', 25, '{"foo":"bar"}', 0, toUnixTimestamp(now()));

insert into scyllax.person(id, email, age, kind, "createdAt") values (e01e880a-414c-11ee-be56-0242ac120002, 'foo2@scyllax.local', 25, 1, toUnixTimestamp(now()));

create table if not exists scyllax.person_login (
	id timeuuid,
	person_id uuid,
	count counter,
	primary key ((id), person_id)
);

update scyllax.person_login set count = count + 0 where id = 42dcfcde-5420-11ee-8c99-0242ac120002 and person_id = e01e84d6-414c-11ee-be56-0242ac120002;
