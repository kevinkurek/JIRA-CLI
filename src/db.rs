use anyhow::Result;
use serde::Deseralize;
use std::fs;

use crate::models{DBState, Story, Epic, Status};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let contenct = fs::
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {

    }
}