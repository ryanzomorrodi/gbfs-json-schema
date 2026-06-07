macro_rules! file_struct {
    ( $ty: ident, $data: ty ) => {
        #[cfg_attr(feature = "napi", napi(object))]
        #[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
        #[serde_with::skip_serializing_none]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct $ty {
            /// Indicates the last time data in the feed was updated. This timestamp represents the publisher's knowledge of the current state of the system at this point in time.
            pub last_updated: Timestamp,
            /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
            pub ttl: u32,
            /// GBFS version number to which the feed conforms, according to the versioning framework.
            pub version: String,
            /// Response data.
            pub data: $data,
        }
    };
}

pub mod free_bike_status;
pub mod station_status;

pub use self::free_bike_status::{FreeBikeStatusData, FreeBikeStatusFile};
pub use self::station_status::{StationStatusData, StationStatusFile};

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    fn test_file<T: serde::de::DeserializeOwned + serde::Serialize>(example: &str) {
        let json_value: serde_json::Value = serde_json::from_str(example).unwrap();

        let file: T = serde_json::from_str(example).unwrap();

        let json_value2: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&file).unwrap()).unwrap();

        assert_eq!(json_value, json_value2);
    }

    #[test]
    fn station_status() {
        let station_status = include_str!("./examples/specification/station_status.json");

        test_file::<super::station_status::StationStatusFile>(station_status);
    }

    #[test]
    fn free_bike_status() {
        let free_bike_status = include_str!("./examples/specification/free_bike_status-1.json");

        test_file::<super::free_bike_status::FreeBikeStatusFile>(free_bike_status);

        let free_bike_status = include_str!("./examples/specification/free_bike_status-2.json");

        test_file::<super::free_bike_status::FreeBikeStatusFile>(free_bike_status);
    }
}
