use diesel::{prelude::*};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Queryable, Debug, Selectable, AsChangeset, Serialize, Default)]
#[diesel(table_name = crate::schema::store::file)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub identifier: String,
    pub size: i64,
    pub mime_type: String,
    // #[serde(skip_serializing)]
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: chrono::NaiveDateTime,
    // #[serde(skip_serializing)]
    #[serde_as(as = "DisplayFromStr")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::store::file)]
pub struct NewFile {
    pub name: String,
    pub identifier: String,
    pub size: i64,
    pub mime_type: String,
}
