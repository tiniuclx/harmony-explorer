-- Your SQL goes here
CREATE TABLE notes (
    chord TEXT NOT NULL,
    degree INTEGER NOT NULL,
    interval INTEGER NOT NULL,
    PRIMARY KEY (chord, interval) 
) WITHOUT ROWID;

CREATE TABLE names (
    chord TEXT NOT NULL,
    alternative_name TEXT PRIMARY KEY
) WITHOUT ROWID;
