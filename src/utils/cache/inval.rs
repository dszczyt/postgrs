use crate::types::oid::{DEFAULTTABLESPACE_OID, Oid};

pub fn get_database_path(db_node: Oid, spc_node: Oid) -> Result<String, String> {
    if spc_node == DEFAULTTABLESPACE_OID {
        return Ok(format!("base/{}", db_node).to_owned());
    }
    Err("Not implemented".to_owned())
}