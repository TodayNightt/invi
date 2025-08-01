INSERT INTO location (id)
VALUES (1);

INSERT INTO image (id, data)
VALUES (1, 'dsdsdfdfdhdfjkhdkh');

INSERT INTO items (id, name, item_metadata, location, image)
VALUES (1, 'Item 1',
        '{"schema_name": "TestSchema", "object_properties_schemas": {}, "values": {"a": "this is a string", "b": 10}}',
        1, 1);

INSERT INTO items (id, name, item_metadata, location, image)
VALUES (2, 'Item 2',
        '{"schema_name": "TestSchema", "object_properties_schemas": {}, "values": {"a": "this is a", "b": 5}}', 1, 1);

INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (1, 1, 1672531200, 0, 30, 30, 0);  -- 2023-01-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (2, 1, 1675123200, 1, 3, 27, 0);   -- 2023-01-31
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (3, 1, 1677628800, 0, 6, 33, 0);   -- 2023-03-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (4, 1, 1685577600, 1, 10, 23, 0);  -- 2023-06-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (5, 2, 1693526400, 0, 30, 30, 0);  -- 2023-09-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (6, 2, 1700000000, 1, 3, 27, 0);   -- 2023-11-15 (your original)
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (7, 2, 1704067200, 0, 6, 33, 0);   -- 2024-01-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (9, 2, 1709251200, 1, 10, 23, 0);  -- 2024-03-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (10, 2, 1717200000, 0, 10, 33, 0); -- 2024-06-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (11, 2, 1725148800, 0, 40, 73, 0); -- 2024-09-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (12, 2, 1730419200, 1, 14, 59, 0); -- 2024-11-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (13, 2, 1733356800, 0, 14, 73, 0); -- 2024-12-05
INSERT INTO records (id, item_id, date, transaction_type, quantity, total, correction)
VALUES (14, 2, 1735689599, 0, 40, 113, 0); -- 2024-12-31