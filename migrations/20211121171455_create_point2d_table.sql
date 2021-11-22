-- Create a table in the `geodb` database.
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