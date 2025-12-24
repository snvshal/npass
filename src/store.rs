use crate::crypto::{decrypt_store, encrypt_store};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Store {
    pub entries: HashMap<String, Entry>,
    pub backups: Vec<Entry>,
}

impl Store {
    pub fn path() -> Result<PathBuf> {
        // Allow overriding store path via NPASS_STORE environment variable (useful for testing)
        if let Ok(p) = std::env::var("NPASS_STORE") {
            return Ok(PathBuf::from(p));
        }
        let proj =
            ProjectDirs::from("com", "example", "npass").context("can't determine config dir")?;
        let d = proj.data_local_dir();
        fs::create_dir_all(d).context("create config dir")?;
        Ok(d.join("store.bin"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        Self::load_from_path(&path)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        self.save_to_path(&path)
    }

    /// Load store from an explicit path (does not rely on env vars). Useful for tests.
    pub fn load_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Store::default());
        }
        let blob = fs::read(path).context("read store file")?;
        if blob.is_empty() {
            return Ok(Store::default());
        }
        let pt = decrypt_store(&blob, "").map_err(|_| anyhow::anyhow!("decrypt store"))?;
        let s: Store = serde_json::from_slice(&pt).context("parse store")?;
        Ok(s)
    }

    /// Save store to an explicit path (does not rely on env vars). Useful for tests.
    pub fn save_to_path<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("create parent dir")?;
        }
        let plain = serde_json::to_vec(self).context("serialize store")?;
        let blob = encrypt_store(&plain, "").map_err(|_| anyhow::anyhow!("encrypt store"))?;
        fs::write(path, blob).context("write store file")?;
        Ok(())
    }

    pub fn set(&mut self, name: &str, value: &str, overwrite: bool) -> anyhow::Result<()> {
        if self.entries.contains_key(name) && !overwrite {
            anyhow::bail!(
                "Entry '{}' already exists. Use '--overwrite' to replace it.",
                name
            );
        }

        let e = Entry {
            name: name.to_string(),
            value: value.to_string(),
        };

        self.entries.insert(name.to_string(), e);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.entries.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Entry> {
        self.entries.remove(name)
    }

    pub fn backup_entry(&mut self, entry: Entry) {
        self.backups.push(entry);
    }

    /// Remove an entry from backups by name. Returns the removed Entry if found.
    pub fn remove_backup(&mut self, name: &str) -> Option<Entry> {
        if let Some(pos) = self.backups.iter().position(|b| b.name == name) {
            Some(self.backups.remove(pos))
        } else {
            None
        }
    }
}
