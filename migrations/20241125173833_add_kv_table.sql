-- Add migration script here
CREATE TABLE kv_store (
    `key` VARCHAR(255) NOT NULL PRIMARY KEY,
    `value` TEXT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;