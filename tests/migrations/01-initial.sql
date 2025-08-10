CREATE TABLE IF NOT EXISTS location_metadata
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    name     TEXT                              NOT NULL,
    metadata TEXT
);

CREATE TABLE IF NOT EXISTS location_data
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    location INTEGER REFERENCES location_metadata (id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    rack     TEXT,
    bin      TEXT
);

CREATE TABLE IF NOT EXISTS image
(
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    key TEXT                              NOT NULL
);

CREATE TABLE IF NOT EXISTS items
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE                                  NOT NULL,
    name          TEXT                                                                      NOT NULL,
    item_metadata TEXT,
    location      INTEGER REFERENCES location_data (id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    image         INTEGER REFERENCES image (id) ON UPDATE CASCADE                           NOT NULL
);


CREATE TABLE IF NOT EXISTS records
(
    id                 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    item_id            INTEGER REFERENCES items (id) ON UPDATE CASCADE,
    date               INTEGER NOT NULL,
    transaction_type   INTEGER NOT NULL CHECK (transaction_type IN (1, 2, 3, 4) ),
    quantity           INTEGER NOT NULL,
    total              INTEGER NOT NULL,
    adjustment_remarks INTEGER REFERENCES records (id) ON UPDATE CASCADE ON DELETE SET NULL
);


