-- Add migration script here
CREATE TABLE quotes (
    quote_id INT AUTO_INCREMENT PRIMARY KEY,          -- Unique ID for each quote
    user_id BIGINT NOT NULL,                          -- Discord user ID (64-bit)
    username VARCHAR(255) NOT NULL,                  -- Username of the person who said the quote
    quote TEXT NOT NULL,                             -- The quote itself
    added_by BIGINT NOT NULL,                        -- Discord user ID of the person who added the quote
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,    -- Timestamp when the quote was added
    INDEX (user_id),                                 -- Index for efficient lookup by user_id
    INDEX (added_by)                                 -- Index for efficient lookup by added_by
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
