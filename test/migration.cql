create keyspace if not exists scyllax_migrations with replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': 3 };

create table if not exists scyllax_migrations.migration (
    bucket int,
	version bigint,
	description text,
	installed_on timestamp,
	success boolean,
	checksum blob,
	execution_time bigint,
	primary key (bucket, version)
);
