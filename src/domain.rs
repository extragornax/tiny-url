/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use crate::schema::data_tiny;

#[derive(Debug, Clone, Insertable, QueryableByName, Queryable, Serialize, Deserialize, Default)]
#[table_name = "data_tiny"]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Data {
    pub id: i64,
    pub base_url: String,
    pub short_url: String,
    pub created_at: NaiveDateTime,
    pub created_by_ip: Option<String>,
}

#[derive(Debug, Clone, Insertable, QueryableByName, Queryable, Serialize, Deserialize)]
#[table_name = "data_tiny"]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DataInsert {
    pub base_url: String,
    pub short_url: String,
    pub created_at: NaiveDateTime,
    pub created_by_ip: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTinyUrl {
    pub url: String,
}
