use std::{env::var, path::PathBuf};

pub fn data_dir() -> PathBuf {
    PathBuf::from(var("PGDATA").unwrap())
}
