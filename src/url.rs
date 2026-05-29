use crate::MutexGuard;
use std::collections::HashMap;
use url::Url;

pub fn dup_url(
    map: &mut MutexGuard<HashMap<String, String>>,
    id: &mut String,
    url: &String,
) -> bool {
    for (key, value) in map.iter() {
        if url == value {
            *id = key.to_string();
            return true;
        }
    }
    return false;
}

pub fn test_dup_url(
    map: &mut HashMap<String, String>,
    id: &mut String,
    url: &String,
) -> bool {
    for (key, value) in map.iter() {
        if url == value {
            *id = key.to_string();
            return true;
        }
    }
    return false;
}

pub fn is_valid_url(url: &String) -> bool {
    match Url::parse(url) {
        Ok(_) => true,
        Err(_) => false,
    }
}
