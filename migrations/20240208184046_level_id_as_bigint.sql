-- Create a temporary table with the correct data types
CREATE TEMPORARY TABLE `songs_tmp` (
    `id` TEXT NOT NULL,
    `name` TEXT,
    `song_name` TEXT NOT NULL,
    `song_id` TEXT NOT NULL,
    `download_url` TEXT NOT NULL,
    `level_id` BIGINT SIGNED,
    `state` TEXT NOT NULL
);

-- Insert data from the existing 'songs' table into the temporary table
INSERT INTO `songs_tmp` SELECT * FROM `songs`;

-- Drop the original 'songs' table
DROP TABLE `songs`;

-- Create the new 'songs' table with the specified schema
CREATE TABLE IF NOT EXISTS `songs` (
    `id` TEXT NOT NULL,
    `name` TEXT,
    `song_name` TEXT NOT NULL,
    `song_id` TEXT NOT NULL,
    `download_url` TEXT NOT NULL,
    `level_id` BIGINT SIGNED,
    `state` TEXT NOT NULL
);

-- Insert data back from the temporary table into the new 'songs' table
INSERT INTO `songs` SELECT * FROM `songs_tmp`;

-- Drop the temporary table
DROP TABLE `songs_tmp`;
