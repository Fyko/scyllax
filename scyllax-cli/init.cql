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
) with clustering order by (version desc)
 	and bloom_filter_fp_chance = 0.01
    and caching = {'keys': 'ALL', 'rows_per_partition': 'ALL'}
    and comment = ''
    and compaction = {'class': 'SizeTieredCompactionStrategy'}
    and compression = {'sstable_compression': 'org.apache.cassandra.io.compress.LZ4Compressor'}
    and crc_check_chance = 1.0
    and dclocal_read_repair_chance = 0.0
    and default_time_to_live = 0
    and gc_grace_seconds = 864000
    and max_index_interval = 2048
    and memtable_flush_period_in_ms = 0
    and min_index_interval = 128
    and read_repair_chance = 0.0
    and speculative_retry = '99.0PERCENTILE';
