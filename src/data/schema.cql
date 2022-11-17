CREATE KEYSPACE test
    with replication = {'class': 'SimpleStrategy', 'replication_factor': '1'}
    AND durable_writes  = true;

x::=CREATE TYPE [ IF NOT EXISTS ] root
    '('

CREATE TABLE test.data (
    success boolean,
    page int,
    total_pages int,
    total_auctions int,
    last_updated int,
)