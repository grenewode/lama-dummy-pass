use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Default, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct Entry {
    pub data: String,
}

impl Entry {
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: (data.into()),
        }
    }

    pub fn passwd(&self) -> Option<&str> {
        self.data.lines().next()
    }

    pub fn meta(&self) -> BTreeMap<&str, &str> {
        self.data
            .lines()
            .filter_map(|line| {
                let mut split = line.splitn(2, ':');
                match (split.next(), split.next()) {
                    (Some(key), Some(value)) => Some((key, value)),
                    _ => None,
                }
            })
            .collect()
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct Database {
    db: BTreeMap<PathBuf, Entry>,
}

impl Database {
    fn normalize(path: impl AsRef<Path>) -> Result<PathBuf> {
        use std::path::Component;
        path.as_ref()
            .components()
            .filter_map(|item| match item {
                Component::ParentDir => Some(Err(Error::InvalidPath(PathBuf::from(path.as_ref())))),
                Component::Normal(text) => Some(Ok(text)),
                _ => None,
            })
            .collect::<Result<PathBuf>>()
    }

    pub fn ls(
        &self,
        start: Option<impl AsRef<Path>>,
    ) -> Result<impl Iterator<Item = (&PathBuf, &Entry)>> {
        let start = start.map(Self::normalize).transpose()?;

        Ok(self.db.iter().filter(move |&(path, _)| match &start {
            Some(start) => path.starts_with(start),
            None => true,
        }))
    }

    pub fn remove(&mut self, start: impl AsRef<Path>) -> Result<()> {
        let to_remove = self
            .ls(Some(start))?
            .map(|item| item.0)
            .cloned()
            .collect::<Vec<_>>();

        for item in to_remove {
            self.db.remove(&item);
        }
        Ok(())
    }

    pub fn entry(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<std::collections::btree_map::Entry<PathBuf, Entry>> {
        Ok(self.db.entry(Self::normalize(path)?))
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Result<Option<&Entry>> {
        Ok(self.db.get(&Self::normalize(path)?))
    }
}

impl FromStr for Database {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&if f.alternate() {
            serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?
        } else {
            serde_json::to_string(self).map_err(|_| fmt::Error)?
        })
    }
}
