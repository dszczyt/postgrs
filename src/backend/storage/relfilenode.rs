use postgrs::types::oid::Oid;

pub struct RelFileNode {
  pub spc_node: Oid,
  pub db_node: Oid,
  pub rel_node: Oid,
}
