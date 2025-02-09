use diesel::{prelude::*};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Queryable, Debug, Insertable, Selectable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::store::file_tag)]
pub struct FileTag {
    pub file_id: i32,
    pub tag_id: i32,
}

#[serde_as]
#[derive(Queryable, Debug, Selectable, AsChangeset, Default, Serialize)]
#[diesel(table_name = crate::schema::store::tag)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: chrono::NaiveDateTime,
    #[serde_as(as = "DisplayFromStr")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::store::tag)]
pub struct NewTag {
    pub name: String,
}