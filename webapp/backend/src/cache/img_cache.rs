use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};
use lazy_static::lazy_static;
use actix_web::web::Bytes;
use std::fs;
use std::path::Path;
use std::process::Command;
use log::error;

#[derive(Clone)]
pub struct ImgCache {
    store: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl ImgCache {
    fn new() -> Self {
        ImgCache {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn set(&self, key: String, value: Bytes, ttl: Option<Duration>) {
        let mut store = self.store.lock().unwrap();
        store.insert(key.clone(), value);

        if let Some(ttl) = ttl {
            let store_clone = Arc::clone(&self.store);
            tokio::spawn(async move {
                time::sleep(ttl).await;
                let mut store = store_clone.lock().unwrap();
                store.remove(&key);
            });
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }
}

lazy_static! {
    pub static ref IMAGE_CACHE: ImgCache = {
        let cache = ImgCache::new();

        // images/user_profile/ 配下の画像をリサイズし、キャッシュする
        let dir = Path::new("images/user_profile");
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let img_name = entry.file_name().to_string_lossy().into_owned();
                    let path = entry.path();
                    if let Ok(output) = Command::new("magick")
                        .arg(&path)
                        .arg("-resize")
                        .arg("500x500")
                        .arg("png:-")
                        .output()
                    {
                        let img_bytes: Bytes = output.stdout.into();
                        let cache_clone = cache.clone();
                        tokio::spawn(async move {
                            cache_clone.set(img_name, img_bytes, None).await;
                        });
                    } else {
                        // Handle command execution error
                        error!("Failed to execute image resize command for {:?}", path);
                    }
                } else {
                    // Handle entry read error
                    error!("Failed to read directory entry");
                }
            }
        } else {
            // Handle directory read error
            error!("Failed to read directory: {:?}", dir);
        }

        cache
    };
}