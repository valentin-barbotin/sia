use diesel::{prelude::*};
use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;

use crate::{getConn, file::{models::File, database::get_file_by_identifier}};


use super::models::*;

pub fn get_tag(tag_id: i32) -> Result<Tag, diesel::result::Error> {
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    let data = tag
    .find(tag_id)
    .first::<Tag>(conn)?;

    Ok(data)
}

pub fn get_tag_by_name(tag_name: &str) -> Result<Tag, diesel::result::Error> {
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    let data = tag
    .filter(name.eq(tag_name))
    .first::<Tag>(conn)?;

    Ok(data)
}

pub fn create_tag(tag_name: &str) -> Result<Tag, diesel::result::Error> {
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    let res = diesel::insert_into(tag)
    .values(
        name.eq(tag_name)
    )
    .get_result(conn)?;

    Ok(res)
}

pub fn remove_tag(tag_name: &str) -> Result<(), diesel::result::Error> {
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    diesel::delete(tag.filter(name.eq(tag_name)))
        .execute(conn)?;

    Ok(())
}

pub fn link_file_with_tag(file: &str, tag_name: &str) -> Result<usize, diesel::result::Error> {
    use crate::schema::store::file_tag::dsl::*;

    let id = get_tag_by_name(tag_name)?.id;
    let file = get_file_by_identifier(file)?.id;

    let conn = getConn!();

    let res = diesel::insert_into(file_tag)
    .values(
        (tag_id.eq(id), file_id.eq(file))
    )
    .execute(conn)?;

    Ok(res)
}

pub fn get_tags() -> Result<Vec<Tag>, diesel::result::Error> {
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    let tags = tag
        .load::<Tag>(conn)?;

    Ok(tags)
}

pub fn get_tags_of_file(file_identifier: &str) -> Result<Vec<Tag>, diesel::result::Error> {
    use crate::schema::store::file_tag::dsl::*;
    use crate::schema::store::tag::dsl::*;

    let conn = getConn!();

    // id from uuid
    let file = get_file_by_identifier(file_identifier)?;

    let tags = file_tag
        .filter(file_id.eq(file.id))
        .inner_join(tag)
        .select(tag::all_columns())
        .load::<Tag>(conn)?;

    Ok(tags)
}

pub fn remove_tag_from_file(file: &str, tag_name: &str) -> Result<(), diesel::result::Error> {
    use crate::schema::store::file_tag::dsl::*;

    let conn = getConn!();

    let id = get_tag_by_name(tag_name)?.id;
    let file = get_file_by_identifier(file)?.id;

    diesel::delete(file_tag.filter(file_id.eq(file).and(tag_id.eq(id))))
        .execute(conn)?;

    Ok(())
}

pub fn get_files_tagged_with(tag_name: &str) -> Result<Vec<File>, diesel::result::Error> {
    use crate::schema::store::file_tag::dsl::*;
    use crate::schema::store::file::dsl::*;

    let conn = getConn!();

    let tag = get_tag_by_name(tag_name)?;

    let files = file_tag
        .filter(tag_id.eq(tag.id))
        .inner_join(file)
        .select(file::all_columns())
        .load::<File>(conn)?;

    Ok(files)
}
