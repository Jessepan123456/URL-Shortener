use crate::url::{is_valid_url, test_dup_url};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_duplicate_url_found() {
        let mut map = HashMap::new();
        map.insert("abc123".to_string(), "https://youtube.com".to_string());

        let mut id = String::new();
        let url = "https://youtube.com".to_string();

        let result = test_dup_url(&mut map, &mut id, &url);

        assert_eq!(result, true);
        assert_eq!(id, "abc123");
    }

    #[test]
    fn test_empty_duplicate_url_found() {
        let mut map = HashMap::new();
        map.insert("abc123".to_string(), "".to_string());

        let mut id = String::new();
        let url = "".to_string();

        let result = test_dup_url(&mut map, &mut id, &url);

        assert_eq!(result, true);
        assert_eq!(id, "abc123");
    }

    #[test]
    fn test_dup_url_not_found() {
        let mut map = HashMap::new();

        let mut id = String::new();
        let url = "https://youtube.com".to_string();

        let result = test_dup_url(&mut map, &mut id, &url);

        assert_eq!(result, false);
        assert_eq!(id, "");
    }

    #[test]
    fn test_valid_url_ok() {
        let url = "https://youtube.com".to_string();
        assert_eq!(is_valid_url(&url), true);
    }

    #[test]
    fn test_valid_url_weird() {
        let url = "https://youtube".to_string();
        assert_eq!(is_valid_url(&url), true);
    }

    #[test]
    fn test_invalid_url_fail() {
        let url = "ahdsa".to_string();
        assert_eq!(is_valid_url(&url), false)
    }

    #[test]
    fn test_invalid_url_empty() {
        let url = "".to_string();
        assert_eq!(is_valid_url(&url), false)
    }
}
