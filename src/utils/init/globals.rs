use std::{path::PathBuf, env::var};

pub fn data_dir() -> PathBuf {
    PathBuf::from(var("PGDATA").unwrap())
}
