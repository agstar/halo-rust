use std::collections::HashMap;
use chrono::{DateTime, Local};

#[warn(dead_code)]
struct Extension {
    api_version: String,
    king: String,
    metadata: Box<dyn MetadataOperator>,
}

trait MetadataOperator {
    fn get_name(self: &Self) -> String;
    fn get_generate_name(self: &Self) -> String;
    fn get_labels(self: &Self) -> String;
    fn get_annotations(self: &Self) -> HashMap<String, String>;
    fn get_version(self: &Self) -> u64;
    fn get_creation_timestamp(self: &Self) -> DateTime<Local>;
}