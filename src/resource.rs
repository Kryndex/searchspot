use serde::de::Deserialize;
use serde::ser::Serialize;

use rs_es::Client;
use rs_es::operations::index::IndexResult;
use rs_es::operations::delete::DeleteResult;
use rs_es::operations::mapping::MappingResult;
use rs_es::error::EsError;

use params::*;

use std::any::Any;
use std::fmt::Debug;

pub trait Resource: Send + Sync + Any + Deserialize + Debug {
  type Results: Serialize + Deserialize;

  /// Respond to GET requests returning an array with found ids
  fn search(mut es: &mut Client, default_index: &str, params: &Map) -> Self::Results;

  /// Respond to POST requests indexing given entity
  fn index(&self, mut es: &mut Client, index: &str) -> Result<IndexResult, EsError>;

  /// Respond to DELETE requests on given id deleting it from given index
  fn delete(mut es: &mut Client, id: &str, index: &str) -> Result<DeleteResult, EsError>;

  /// Respond to DELETE requests rebuilding and reindexing given index
  fn reset_index(mut es: &mut Client, index: &str) -> Result<MappingResult, EsError>;
}
