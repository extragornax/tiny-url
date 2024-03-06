/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use chrono::{NaiveDateTime, Utc};

pub fn get_current_datetime() -> NaiveDateTime {
    Utc::now().naive_utc()
}
