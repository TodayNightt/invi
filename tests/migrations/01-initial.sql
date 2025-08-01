CREATE TABLE IF NOT EXISTS location
(
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS image
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    data BLOB                              NOT NULL
);

CREATE TABLE IF NOT EXISTS items
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE                             NOT NULL,
    name          TEXT                                                                 NOT NULL,
    item_metadata TEXT,
    location      INTEGER REFERENCES location (id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    image         INTEGER REFERENCES image (id) ON UPDATE CASCADE                      NOT NULL
);


CREATE TABLE IF NOT EXISTS records
(
    id               INTEGER                                     NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    item_id          INTEGER REFERENCES items (id) ON UPDATE CASCADE,
    date             INTEGER                                     NOT NULL,
    transaction_type INTEGER                                     NOT NULL CHECK (transaction_type IN (0, 1) ),
    quantity         INTEGER                                     NOT NULL,
    total            INTEGER                                     NOT NULL,
    correction       INTEGER CHECK (transaction_type IN (0, 1) ) NOT NULL
);


