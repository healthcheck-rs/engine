CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE healthchecks (
    id uuid DEFAULT uuid_generate_v4 (), 
    site TEXT NOT NULL, 
    status BOOLEAN NOT NULL,
    latency INT, 
    query_time TIMESTAMP NOT NULL,

    PRIMARY KEY (id)
)