BEGIN;

DROP TABLE IF EXISTS build;

CREATE TABLE build (
    id TEXT PRIMARY KEY,
    build_number INTEGER NOT NULL,
    release_date TEXT NOT NULL
);

COMMIT;