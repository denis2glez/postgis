-- Enables the postgis extension in the `testdb` database specified in the `.env` file.
CREATE EXTENSION postgis;
CREATE TABLE point2d (
    fid SERIAL,
    intfield integer,
    strfield character varying,
    realfield double precision,
    datetimefield timestamp with time zone,
    datefield date,
    binaryfield bytea,
    geom geometry(Point)
);