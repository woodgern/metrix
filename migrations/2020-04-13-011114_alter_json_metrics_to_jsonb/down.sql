-- This file should undo anything in `up.sql`
ALTER TABLE metrics ALTER COLUMN data type JSON;
