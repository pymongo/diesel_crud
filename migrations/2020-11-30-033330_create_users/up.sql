CREATE TABLE users (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	email TEXT NOT NULL UNIQUE,
	-- TIMESTAMP, DATETIME is store as INTEGER type without timezone in sqlite
	-- SQLite's TIMESTAMP type is map to Timestamp in diesel_schema
	-- [SQLite Current Timestamp with Milliseconds?](https://stackoverflow.com/questions/17574784/sqlite-current-timestamp-with-milliseconds)
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);