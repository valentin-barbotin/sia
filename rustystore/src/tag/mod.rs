pub mod controllers;
pub mod grpc;
pub mod tag;
pub mod database;
pub mod models;


pub mod storefile {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/storefile.v1.rs"));
    }
}