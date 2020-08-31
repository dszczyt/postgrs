use crate::backend::storage::buffer::ForkNumber;
use crate::backend::storage::relfilenode::RelFileNode;
use crate::backend::storage::relfilenodebackend::RelFileNodeBackend;
use crate::backend::storage::INVALID_BACKEND_ID;
use crate::types::oid::{Oid, DEFAULTTABLESPACE, GLOBALTABLESPACE};

pub const PG_MAJORVERSION: usize = 12;
pub const CATALOG_VERSION_NO: usize = 202004241;
pub const TABLESPACE_VERSION_DIRECTORY: String =
  format!("PG_{}_{}", PG_MAJORVERSION, CATALOG_VERSION_NO);

pub const FORK_NAMES: &[&str] = &["main", "fsm", "vm", "init"];

#[inline]
pub fn relpathbackend(r: RelFileNode, backend_id: i32, forknum: ForkNumber) -> String {
  get_relation_path(r.db_node, r.spc_node, r.rel_node, backend_id, forknum)
}

#[inline]
pub fn relpath(r: RelFileNodeBackend, forknum: ForkNumber) -> String {
  relpathbackend(r.node, r.backend_id, forknum)
}

pub fn get_relation_path(
  db_node: Oid,
  spc_node: Oid,
  rel_node: Oid,
  backend_id: i32,
  forknum: ForkNumber,
) -> String {
  match spc_node {
    GLOBALTABLESPACE => match forknum {
      ForkNumber::Main => format!("global/{}", rel_node),
      _ => format!("global/{}_{}", rel_node, FORK_NAMES[forknum as usize]),
    },
    DEFAULTTABLESPACE => match backend_id {
      INVALID_BACKEND_ID => match forknum {
        ForkNumber::Main => format!("base/{}/{}", db_node, rel_node),
        _ => format!(
          "base/{}/{}_{}",
          db_node, rel_node, FORK_NAMES[forknum as usize]
        ),
      },
      _ => match forknum {
        ForkNumber::Main => format!("base/{}/t{}_{}", db_node, backend_id, rel_node),
        _ => format!(
          "base/{}/t{}_{}_{}",
          db_node, backend_id, rel_node, FORK_NAMES[forknum as usize]
        ),
      },
    },
    _ => match backend_id {
      INVALID_BACKEND_ID => match forknum {
        ForkNumber::Main => format!(
          "pg_tblspc/{}/{}/{}/{}",
          spc_node, TABLESPACE_VERSION_DIRECTORY, db_node, rel_node
        ),
        _ => format!(
          "pg_tblspc/{}/{}/{}/{}_{}",
          spc_node, TABLESPACE_VERSION_DIRECTORY, db_node, rel_node, FORK_NAMES[forknum as usize]
        ),
      },
      _ => match forknum {
        ForkNumber::Main => format!(
          "pg_tblspc/{}/{}/{}/t{}_{}",
          spc_node, TABLESPACE_VERSION_DIRECTORY, db_node, backend_id, rel_node
        ),
        _ => format!(
          "pg_tblspc/{}/{}/{}/t{}_{}_{}",
          spc_node,
          TABLESPACE_VERSION_DIRECTORY,
          db_node,
          backend_id,
          rel_node,
          FORK_NAMES[forknum as usize]
        ),
      },
    },
  }
}
