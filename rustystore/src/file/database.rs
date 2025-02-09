use diesel::{prelude::*};
use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;

use crate::getConn;


use super::models::*;

pub fn insert_file(data: &NewFile) -> Result<i32, diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    let res = diesel::insert_into(file)
        .values(data)
        .get_result::<File>(conn)?;

    Ok(res.id)
}

pub fn remove_file_by_identifier(file_identifier: &str) -> Result<(), diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    diesel::delete(file.filter(identifier.eq(file_identifier)))
        .execute(conn)?;

    Ok(())
}

pub fn remove_file_by_id(file_id: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    diesel::delete(file.filter(id.eq(file_id)))
        .execute(conn)?;

    Ok(())
}

pub fn get_file_by_identifier(file_identifier: &str) -> Result<File, diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    let data = file
        .filter(identifier.eq(file_identifier))
        .first::<File>(conn)?;

    Ok(data)
}

pub fn get_file_by_id(file_id: i32) -> Result<File, diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    let data = file
        .filter(id.eq(file_id))
        .first::<File>(conn)?;

    Ok(data)
}

pub fn get_files() -> Result<Vec<File>, diesel::result::Error> {
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    let files = file
        .load::<File>(conn)?;

    Ok(files)
}
