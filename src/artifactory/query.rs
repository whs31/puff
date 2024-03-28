use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageQueryResponseEntry
{
  pub repo: String,
  pub path: String,
  pub name: String,
  #[serde(rename = "type")] pub type_field: String,
  pub size: i64,
  pub created: String,
  pub modified: String,
  pub modified_by: String,
  pub updated: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageQueryResponseRange
{
  pub start_pos: i64,
  pub end_pos: i64,
  pub total: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageQueryResponse
{
  pub results: Vec<PackageQueryResponseEntry>,
  pub range: Option<PackageQueryResponseRange>
}