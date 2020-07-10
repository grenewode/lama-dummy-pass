use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Default, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct Store {
    store: BTreeMap<PathBuf, String>,
}

impl Store {
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

    pub fn list(
        &self,
        start: Option<impl AsRef<Path>>,
    ) -> Result<impl Iterator<Item = (&Path, &str)>> {
        let start = start.map(Self::normalize).transpose()?;

        Ok(self
            .store
            .iter()
            .filter(move |&(path, _)| match &start {
                Some(start) => path.starts_with(start),
                None => true,
            })
            .map(|(path, value)| (path.as_path(), value.as_str())))
    }

    pub fn list_paths(
        &self,
        start: Option<impl AsRef<Path>>,
    ) -> Result<impl Iterator<Item = &Path>> {
        Ok(self.list(start)?.map(|item| item.0))
    }

    pub fn show(&self, path: Option<impl AsRef<Path>>) -> Result<()> {
        use std::io::prelude::*;

        let path = path.map(Self::normalize).transpose()?;
        let children = self.list(path.as_ref())?.collect::<Vec<_>>();

        match children.as_slice() {
            [] => {
                if let Some(path) = path {
                    return Err(Error::NotInStore(path));
                } else {
                    println!("Password Store")
                }
            }
            [item] if path.as_ref().map(|path| path == item.0).unwrap_or(false) => {
                std::io::stdout().write_all(item.1.as_bytes())?;
                std::io::stdout().flush()?;
            }
            _ => {
                let mut most_recently_printed_ancestor = path.unwrap_or_default();

                let children = children.into_iter();

                let print_item = |parent: &Path, item: &std::path::Component| {
                    for _ in parent {
                        print!("\t");
                    }

                    println!("{item}", item = AsRef::<Path>::as_ref(item).display());
                };

                // If are printing the whole store, then say so
                if most_recently_printed_ancestor.components().next().is_none() {
                    println!("Password Store");
                } else {
                    println!("{}", most_recently_printed_ancestor.display())
                }

                for (child, _) in children {
                    // We want to find the longest path that has already been printed
                    // Once we have that, we can calculate how this child should be
                    // indented
                    let longest_printed_common_path: PathBuf = Iterator::zip(
                        most_recently_printed_ancestor.components(),
                        child.components(),
                    )
                    .take_while(|(a, b)| a == b)
                    .map(|seg| seg.1)
                    .collect();

                    // Now we print out the path elements between the longest common path
                    // and this child
                    if let Ok(unprinted_path_elements) =
                        child.strip_prefix(&longest_printed_common_path)
                    {
                        most_recently_printed_ancestor = longest_printed_common_path;
                        for path_element in unprinted_path_elements.components() {
                            print_item(&most_recently_printed_ancestor, &path_element);
                            most_recently_printed_ancestor.push(path_element);
                        }
                    } else {
                        most_recently_printed_ancestor.clear();

                        for path_element in child.components() {
                            print_item(&most_recently_printed_ancestor, &path_element);
                            most_recently_printed_ancestor.push(path_element);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, start: impl AsRef<Path>) -> Result<()> {
        let to_remove = self
            .list_paths(Some(start))?
            .map(PathBuf::from)
            .collect::<Vec<_>>();

        for item in to_remove {
            self.store.remove(&item);
        }

        Ok(())
    }

    pub fn entry(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<std::collections::btree_map::Entry<PathBuf, String>> {
        Ok(self.store.entry(Self::normalize(path)?))
    }
}

impl FromStr for Store {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&if f.alternate() {
            serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?
        } else {
            serde_json::to_string(self).map_err(|_| fmt::Error)?
        })
    }
}
