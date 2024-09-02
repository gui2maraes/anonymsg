pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;

use domain::namemap::NameMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type NameMapState = Arc<RwLock<NameMap>>;
