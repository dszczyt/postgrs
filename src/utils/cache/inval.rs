use crate::types::oid::{Oid, DEFAULTTABLESPACE_OID};

pub fn get_database_path(db_node: Oid, spc_node: Oid) -> Result<String, String> {
    if spc_node == DEFAULTTABLESPACE_OID {
        return Ok(format!("base/{}", db_node));
    }
    Err("Not implemented".to_owned())
}
