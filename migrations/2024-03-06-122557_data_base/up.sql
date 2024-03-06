/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

CREATE TABLE IF NOT EXISTS data_tiny (
     id BIGSERIAL PRIMARY KEY NOT NULL,
     base_url VARCHAR NOT NULL,
     short_url VARCHAR NOT NULL,
     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
     created_by_ip VARCHAR DEFAULT '0.0.0.0'
);
