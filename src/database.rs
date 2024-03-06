/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use crate::domain::{Data, DataInsert};
use crate::schema::data_tiny::dsl::data_tiny;
use crate::schema::data_tiny::{base_url, created_at, created_by_ip, id, short_url};

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn db_get_tiny_url(_url: String) -> Result<Data, diesel::result::Error> {
    use crate::schema::data_tiny::dsl::*;

    let connection = &mut establish_connection();

    data_tiny
        .filter(short_url.eq(_url))
        .select((
            id,
            base_url,
            short_url,
            created_at,
            created_by_ip,
        ))
        .first(connection)
}

pub fn db_create_tiny_url(_data: DataInsert) -> Result<Data, diesel::result::Error> {
    use crate::schema::data_tiny::dsl::*;
    let connection = &mut establish_connection();

    diesel::insert_into(data_tiny)
        .values(&_data)
        .get_result(connection)
}
