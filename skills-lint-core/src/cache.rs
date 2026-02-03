use std::collections::HashMap;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::errors::LintError;
use crate::tokenizer;

const CACHE_DIR: &str = ".skills-lint-cache";
const CACHE_FILE: &str = "tokens.json";
const CACHE_VERSION: u64 = 1;

pub struct TokenCache {
    entries: HashMap<String, usize>,
    dirty: bool,
}

impl TokenCache {
    /// Load cache from disk. Returns an empty cache if the file is missing, corrupt, or wrong version.
    pub fn load() -> Self {
        let path = Path::new(CACHE_DIR).join(CACHE_FILE);
        let entries = (|| -> Option<HashMap<String, usize>> {
            let content = std::fs::read_to_string(&path).ok()?;
            let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
            let obj = parsed.as_object()?;
            if obj.get("v")?.as_u64()? != CACHE_VERSION {
                return None;
            }
            let entries_obj = obj.get("entries")?.as_object()?;
            let mut map = HashMap::new();
            for (k, v) in entries_obj {
                map.insert(k.clone(), v.as_u64()? as usize);
            }
            Some(map)
        })()
        .unwrap_or_default();

        TokenCache {
            entries,
            dirty: false,
        }
    }

    /// Count tokens for the given text and encoding, using the cache when possible.
    pub fn count_tokens(&mut self, text: &str, encoding: &str) -> Result<usize, LintError> {
        let key = cache_key(text, encoding);
        if let Some(&count) = self.entries.get(&key) {
            return Ok(count);
        }
        let count = tokenizer::count_tokens(text, encoding)?;
        self.entries.insert(key, count);
        self.dirty = true;
        Ok(count)
    }

    /// Write cache to disk if dirty. Errors are silently ignored.
    pub fn flush(&self) {
        if !self.dirty {
            return;
        }
        let _ = (|| -> Result<(), Box<dyn std::error::Error>> {
            std::fs::create_dir_all(CACHE_DIR)?;
            let entries: serde_json::Value = self
                .entries
                .iter()
                .map(|(k, &v)| (k.clone(), serde_json::Value::from(v)))
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into();
            let doc = serde_json::json!({
                "v": CACHE_VERSION,
                "entries": entries,
            });
            let json = serde_json::to_string_pretty(&doc)?;
            std::fs::write(Path::new(CACHE_DIR).join(CACHE_FILE), format!("{json}\n"))?;
            Ok(())
        })();
    }
}

fn cache_key(text: &str, encoding: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let hash = hasher.finalize();
    let hex = &format!("{hash:x}")[..16];
    format!("{hex}:{encoding}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_format() {
        let key = cache_key("hello", "cl100k_base");
        assert!(key.ends_with(":cl100k_base"));
        // 16 hex chars + colon + encoding name
        let parts: Vec<&str> = key.splitn(2, ':').collect();
        assert_eq!(parts[0].len(), 16);
        assert_eq!(parts[1], "cl100k_base");
    }

    #[test]
    fn test_cache_hit() {
        let mut cache = TokenCache {
            entries: HashMap::new(),
            dirty: false,
        };
        let count1 = cache.count_tokens("Hello, world!", "cl100k_base").unwrap();
        assert!(!cache.entries.is_empty());
        assert!(cache.dirty);
        let count2 = cache.count_tokens("Hello, world!", "cl100k_base").unwrap();
        assert_eq!(count1, count2);
    }

    #[test]
    fn test_load_empty() {
        let cache = TokenCache::load();
        // Should not panic, just return empty
        assert!(!cache.dirty);
    }
}
