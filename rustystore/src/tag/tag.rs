use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;

use crate::file::models::File;

#[cfg(not(test))]
use super::database;

#[cfg(test)]
use tests::database;

use super::{
    controllers::{
    },
    models::{
        Tag
    }
};

#[named]
pub fn create_tag(tag_name: &str) -> Result<Tag, diesel::result::Error> {
    if tag_name.trim().is_empty() {
        panic!("Tagname is empty");
    }

    match database::create_tag(tag_name) {
        Ok(tag) => {
            Ok(tag)
        }
        Err(e) => {
            Err(e)
        },
    }
}

pub fn remove_tag(tag_name: &str) -> Result<(), diesel::result::Error> {
    if tag_name.trim().is_empty() {
        panic!("Tag name is empty");
    }

    match database::remove_tag(tag_name) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        },
    }
}
pub fn link_file_with_tag(file_id: &str, tag_name: &str) -> Result<(), diesel::result::Error> {
    if file_id.trim().is_empty() {
        panic!("File id is empty");
    }
    if tag_name.trim().is_empty() {
        panic!("Tag name is empty");
    }

    match database::link_file_with_tag(file_id, tag_name) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        },
    }
}
pub fn get_tags() -> Result<Vec<Tag>, diesel::result::Error> {
    match database::get_tags() {
        Ok(tags) => {
            Ok(tags)
        }
        Err(e) => {
            Err(e)
        },
    }
}

pub fn get_tags_of_file(file_id: &str) -> Result<Vec<Tag>, diesel::result::Error> {
    if file_id.trim().is_empty() {
        panic!("File id is empty");
    }

    match database::get_tags_of_file(file_id) {
        Ok(tags) => {
            Ok(tags)
        }
        Err(e) => {
            Err(e)
        },
    }
}
pub fn remove_tag_from_file(file_id: &str, tag_name: &str) -> Result<(), diesel::result::Error> {
    if file_id.trim().is_empty() {
        panic!("File id is empty");
    }
    if tag_name.trim().is_empty() {
        panic!("Tag name is empty");
    }

    match database::remove_tag_from_file(file_id, tag_name) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        },
    }
}
pub fn get_files_tagged_with(tag_name: &str) -> Result<Vec<File>, diesel::result::Error> {
    if tag_name.trim().is_empty() {
        panic!("Tag name is empty");
    }

    match database::get_files_tagged_with(tag_name) {
        Ok(files) => {
            Ok(files)
        }
        Err(e) => {
            Err(e)
        },
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    use rstest::*;

    pub mod database {
        use crate::file::models::File;
        use crate::tag::models::Tag;

        use super::*;

        pub fn create_tag(tag_name: &str) -> Result<Tag, diesel::result::Error> {
            Ok(Tag::default())
        }

        pub fn link_file_with_tag(file_id: &str, tag_name: &str) -> Result<(), diesel::result::Error> {
            Ok(())
        }

        pub fn get_tags() -> Result<Vec<Tag>, diesel::result::Error> {
            Ok(
                vec![
                    Tag::default(),
                    Tag::default()
                ]
            )
        }

        pub fn get_tags_of_file(file_id: &str) -> Result<Vec<Tag>, diesel::result::Error> {
            Ok(
                vec![
                    Tag::default(),
                    Tag::default()
                ]
            )
        }

        pub fn remove_tag(tag_name: &str) -> Result<(), diesel::result::Error> {
            Ok(())
        }

        pub fn remove_tag_from_file(file_id: &str, tag_name: &str) -> Result<(), diesel::result::Error> {
            Ok(())
        }

        pub fn get_files_tagged_with(tag_name: &str) -> Result<Vec<File>, diesel::result::Error> {
            Ok(
                vec![
                    File::default(),
                    File::default()
                ]
            )
        }
    }

    #[rstest]
    #[test]
    #[case::valid("tag1")]
    #[should_panic(expected="Tagname is empty")]
    #[case::invalid_empty("")]
    fn try_to_create_tag(#[case] tag_name: &str) {
        let res = create_tag(tag_name);

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    #[case::valid("abc", "tag1")]
    #[should_panic(expected="File id is empty")]
    #[case::invalid_file("", "tag1")]
    #[should_panic(expected="Tag name is empty")]
    #[case::invalid_tag("abc", "")]
    fn try_link_file_with_tag(#[case] file_id: &str, #[case] tag_name: &str) {
        let res = link_file_with_tag(file_id, tag_name);

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    #[case::valid("tag1")]
    #[should_panic(expected="Tag name is empty")]
    #[case::invalid_empty("")]
    fn try_remove_tag(#[case] tag_name: &str) {
        let res = remove_tag(tag_name);

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    fn try_get_tags() {
        let res = get_tags();

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    #[case::valid("tag1")]
    #[should_panic(expected="File id is empty")]
    #[case::invalid_negative("")]
    fn try_get_tags_of_file(#[case] file_identifier: &str) {
        let res = get_tags_of_file(file_identifier);

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    #[case::valid("tag1", "abc")]
    #[should_panic(expected="Tag name is empty")]
    #[case::invalid_empty("", "abc")]
    #[should_panic(expected="File id is empty")]
    #[case::invalid_negative("tag1", "")]
    fn try_remove_tag_from_file(#[case] tag_name: &str, #[case] file_id: &str) {
        let res = remove_tag_from_file(file_id, tag_name);

        assert!(res.is_ok());
    }

    #[rstest]
    #[test]
    #[case::valid("tag1")]
    #[should_panic(expected="Tag name is empty")]
    #[case::invalid_empty("")]
    fn try_get_files_tagged_with(#[case] tag_name: &str) {
        let res = get_files_tagged_with(tag_name);

        assert!(res.is_ok());
    }
    
}