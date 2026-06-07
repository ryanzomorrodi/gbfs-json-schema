pub mod files;
pub mod types;
pub mod urls;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "napi")]
use napi_derive::napi;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi(object))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GbfsObjects {
    // vehicle_status
    // station_status
    // station_information
    // vehicle_types
    // system_pricing_plans
}
