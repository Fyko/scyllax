-- create keyspace if not exists scyllax_migrations WITH replication = {'class': 'NetworkTopologyStrategy', 'datacenter1': 3 } AND durable_writes = true;
create keyspace if not exists scyllax_migrations with replication = { 'class': 'LocalStrategy' };
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
