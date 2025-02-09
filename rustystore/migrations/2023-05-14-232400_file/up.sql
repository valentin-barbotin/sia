-- Your SQL goes here

-- CREATE SCHEMA IF NOT EXISTS store;
-- CREATE USER store WITH PASSWORD 'store';
-- DO
-- $do$
-- BEGIN
--    IF NOT EXISTS (
--       SELECT FROM pg_catalog.pg_roles
--       WHERE  rolname = 'store') THEN
--       CREATE USER store WITH PASSWORD 'store';
--    END IF;
-- END
-- $do$;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA store to store;

CREATE TABLE IF NOT EXISTS store.file (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    identifier VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS file_name ON store.file (name);
CREATE UNIQUE INDEX IF NOT EXISTS file_identifier ON store.file (identifier);

CREATE TABLE IF NOT EXISTS store.tag (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS tag_name ON store.tag (name);


CREATE TABLE IF NOT EXISTS store.file_tag (
    file_id INT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY (file_id, tag_id),
    FOREIGN KEY (file_id) REFERENCES store.file (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES store.tag (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS file_tag_file_id ON store.file_tag (file_id);
CREATE INDEX IF NOT EXISTS file_tag_tag_id ON store.file_tag (tag_id);

