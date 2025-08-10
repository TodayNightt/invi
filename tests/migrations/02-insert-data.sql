INSERT INTO location_metadata (id, name, metadata)
VALUES (1, 'Container 1', '{"schema_name": "TestSchema","object_properties_schemas": {},"values": {"racks": [{"rack_name": "Rack A","shelves": ["Shelf α","Shelf β","Shelf γ"]},{"rack_name": "Rack B","shelves": ["Shelf δ","Shelf ε","Shelf ζ"]},{"rack_name": "Rack C","shelves": ["Shelf θ","Shelf ι"]}]}}');
INSERT INTO location_metadata (id, name) VALUES (2, 'Hall 1');
INSERT INTO location_data (id, location, rack, bin)
VALUES (1, 1, 'Rack 1', 'Bin 1');
INSERT INTO location_data (id, location) VALUES (2, 2);

INSERT INTO image (id, key)
VALUES (1, 'abd0031');
INSERT INTO image (id, key)
VALUES (2, 'djs0032');
INSERT INTO image (id, key)
VALUES (3, 'dss0033');

INSERT INTO items (id, name, item_metadata, location, image)
VALUES (1, 'Item 1',
        '{"schema_name": "TestSchema", "object_properties_schemas": {}, "values": {"a": "this is a string", "b": 10}}',
        1, 1);

INSERT INTO items (id, name, item_metadata, location, image)
VALUES (2, 'Item 2',
        '{"schema_name": "TestSchema", "object_properties_schemas": {}, "values": {"a": "this is a", "b": 5}}', 1, 1);

INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (1, 1, 1672531200, 1, 30, 30); -- 2023-01-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (2, 1, 1675123200, 2, 3, 27); -- 2023-01-31
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (3, 1, 1677628800, 1, 6, 33); -- 2023-03-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (4, 1, 1685577600, 2, 10, 23); -- 2023-06-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (5, 2, 1693526400, 1, 30, 30); -- 2023-09-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (6, 2, 1700000000, 2, 3, 27); -- 2023-11-15
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (7, 2, 1704067200, 1, 6, 33); -- 2024-01-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (9, 2, 1709251200, 2, 10, 23); -- 2024-03-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (10, 2, 1717200000, 1, 10, 33); -- 2024-06-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (11, 2, 1725148800, 1, 40, 73); -- 2024-09-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (12, 2, 1730419200, 2, 14, 59); -- 2024-11-01
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (13, 2, 1733356800, 1, 14, 73); -- 2024-12-05
INSERT INTO records (id, item_id, date, transaction_type, quantity, total)
VALUES (14, 2, 1735689599, 1, 40, 113); -- 2024-12-31