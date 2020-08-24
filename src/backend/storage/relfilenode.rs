use postgrs::types::oid::Oid;

pub struct RelFileNode {
  pub SpecNode: Oid,
  pub DBNode: Oid,
  pub RelNode: Oid,
}
