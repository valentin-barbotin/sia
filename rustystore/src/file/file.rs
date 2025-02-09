use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;

#[cfg(not(test))]
use super::database;

#[cfg(test)]
use tests::database;

use super::{
    controllers::{
        FileInfo
    },
    models::{
        File as FileModel,
        NewFile,
    }
};

#[named]
pub fn insert(data: &FileInfo) -> Result<(), diesel::result::Error> {
    let name = data.name.to_owned();
    let identifier = data.identifier.to_owned();
    let size = data.size.to_owned();
    let mime_type = data.mime_type.to_owned();

    if name.trim().is_empty() {
        panic!("Name is empty");
    }
    if identifier.trim().is_empty() {
        panic!("Identifier is empty");
    }
    if size < 0 {
        panic!("Size can't be negative");
    }
    if mime_type.trim().is_empty() {
        panic!("Mime_type is empty");
    }

    let file = NewFile {
        name,
        identifier,
        size,
        mime_type,
    };

    match database::insert_file(&file) {
        Ok(_) => {
            data.tags.iter().for_each(|tag| {
                if tag.trim().is_empty() {
                    error!("[{}] - Empty tag in tag list", function_name!());
                }

                match crate::tag::tag::create_tag(tag) {
                    Ok(_) => {
                        info!("[{}] - Tag inserted", function_name!());
                    }
                    Err(e) => {
                        if e.to_string().contains("duplicate key value") {
                            info!("[{}] - Tag already exists", function_name!());

                            match crate::tag::tag::link_file_with_tag(&file.identifier, tag) {
                                Ok(_) => {
                                    info!("[{}] - File linked with tag", function_name!());
                                }
                                Err(e) => {
                                    error!("[{}] - {}", function_name!(), e);
                                },
                            }
                        } else {
                            error!("[{}] - {}", function_name!(), e);
                        }
                    },
                }
            });

            Ok(())
        }
        Err(e) => {
            Err(e)
        },
    }
}

#[named]
pub fn remove(identifier: &str) -> Result<(), diesel::result::Error> {
    if identifier.trim().is_empty() {
        panic!("Identifier is empty");
    }

    // Check if file exists
    let exists = database::get_file_by_identifier(identifier);
    if exists.is_err() {
        warn!("[{}] - File {} does not exist", function_name!(), identifier);
    }

    match database::remove_file_by_identifier(identifier) {
        Ok(_) => {
            // info!("[{}] - File removed", function_name!());
            Ok(())
        }
        Err(e) => {
            // error!("[{}] - {}", function_name!(), e);
            Err(e)
        },
    }
}

#[named]
pub fn get(identifier: &str) -> Result<FileModel, diesel::result::Error> {
    if identifier.trim().is_empty() {
        panic!("Identifier is empty");
    }

    match database::get_file_by_identifier(identifier) {
        Ok(file) => {
            // info!("[{}] - File retrieved", function_name!());
            Ok(file)
        }
        Err(e) => {
            // warn!("[{}] - {}", function_name!(), e);
            Err(e)
        },
    }
}

#[named]
pub fn get_all() -> Result<Vec<FileModel>, diesel::result::Error> {
    match database::get_files() {
        Ok(files) => {
            // info!("[{}] - Files retrieved", function_name!());
            Ok(files)
        }
        Err(e) => {
            // error!("[{}] - {}", function_name!(), e);
            Err(e)
        },
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    use rstest::*;

    // mock
    pub mod database {
        use super::*;
        use super::super::super::models::File;

        pub fn insert_file(data: &NewFile) -> Result<i32, diesel::result::Error> {
            Ok(1)
        }
        pub fn remove_file_by_identifier(file_identifier: &str) -> Result<(), diesel::result::Error> {
            Ok(())
        }
        pub fn remove_file_by_id(file_id: i32) -> Result<(), diesel::result::Error> {
            Ok(())
        }
        pub fn get_file_by_identifier(file_identifier: &str) -> Result<File, diesel::result::Error> {
            Ok(File::default())
        }
        pub fn get_file_by_id(file_id: i32) -> Result<File, diesel::result::Error> {
            Ok(File::default())
        }
        pub fn get_files() -> Result<Vec<File>, diesel::result::Error> {
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
    #[case::valid_with_empty_tags("file1.txt", "abcd", 5, "text/plain", vec![])]
    #[should_panic(expected="Size can't be negative")]
    #[case::negative_size("file1.txt", "abcd", -10, "text/plain", vec![])]
    #[should_panic(expected="Name is empty")]
    #[case::empty_param2("", "abcd", 5, "text/plain", vec![])]
    #[should_panic(expected="Identifier is empty")]
    #[case::empty_param3("file1.txt", "", 5, "text/plain", vec![])]
    #[should_panic(expected="Mime_type is empty")]
    #[case::empty_param4("file1.txt", "abcd", 5, "", vec![])]
    fn try_to_insert(#[case] name: String, #[case] identifier: String, #[case] size: i64, #[case] mime_type: String, #[case] tags: Vec<String>) {
        let data = FileInfo {
            name,
            identifier,
            size,
            mime_type,
            tags
        };

        // assert fn throw error
        insert(&data).unwrap();
    }

    #[rstest]
    #[test]
    #[case::valid_identifier("1234_5678")]
    #[should_panic(expected="Identifier is empty")]
    #[case::invalid_identifier("")]
    fn try_to_remove(#[case] identifier: &str) {
        remove(identifier).unwrap();
    }

    #[rstest]
    #[test]
    #[case::valid_identifier("1234_5678")]
    #[should_panic(expected="Identifier is empty")]
    #[case::invalid_identifier("")]
    fn try_to_get(#[case] identifier: &str) {
        get(identifier).unwrap();
    }

    #[rstest]
    #[test]
    fn try_to_get_all() {
        get_all().unwrap();
    }
}
