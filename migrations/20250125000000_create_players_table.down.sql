-- Add down migration script here
DROP INDEX IF EXISTS uniq_single_owner;
DROP TABLE IF EXISTS players;