use std::env::var;

pub fn data_dir() -> String {
    var("PGDATA").unwrap()
}
