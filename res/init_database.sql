-- This script contains all the initialization procedures for the Portal 
-- Service's database.
--
-- To appy changes to an existing database, just run this script once again, it 
-- will execute all the ALTER procedures it contains without whipping off the 
-- existing data.
--
-- Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
 
 -- Creates the Portal Service's database and its tables:
 -- # Tables:
 -- * portal
 -- * portal_content
 -- * portal_current
CREATE DATABASE portaldb;

-- Go to characterdb
\c portaldb

CREATE TABLE IF NOT EXISTS portal (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50),
    description TEXT,
    image_url TEXT
);

CREATE TABLE IF NOT EXISTS portal_content (
	portal BIGINT REFERENCES portal(id),
	character BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS portal_current (
	portal BIGINT REFERENCES portal(id)
);

