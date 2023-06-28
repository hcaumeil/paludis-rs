use std::path::PathBuf;

use cxx::SharedPtr;

use super::bindings::paludis_repository_category_names;
use super::bindings::paludis_repository_metadata_exist;
use super::bindings::paludis_repository_metadata_key;
use super::bindings::paludis_repository_metadata_names;
use super::bindings::paludis_repository_name;
use super::bindings::paludis_repository_package_id_from_canonical_form;
use super::bindings::paludis_repository_package_ids_canonical_form;
use super::bindings::paludis_repository_package_names;

use super::metadata::new_metadata_key;
use super::packageid::new_package_id;
use super::Environment;
use super::MetadataKey;
use super::MetadataKeyType;
use super::PackageID;

/// A Repository provides a representation of a physical repository to an [`Environment`]
pub struct Repository {
    name: String,
    ptr: SharedPtr<u64>,
}

pub fn new_repository(ptr: SharedPtr<u64>) -> Repository {
    let name = repository_name(ptr.to_owned());
    Repository { ptr, name }
}

fn repository_name(ptr: SharedPtr<u64>) -> String {
    paludis_repository_name(ptr)
}

impl Repository {
    /// Returns repository name.
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    /// Fetch category names.
    pub fn category_names(&self) -> Vec<String> {
        paludis_repository_category_names(self.ptr.to_owned())
    }

    /// Returns the names of all packages within the given category    
    pub fn package_names_by_category(&self, category: &str) -> Vec<String> {
        paludis_repository_package_names(self.ptr.to_owned(), category)
    }

    /// Fetch all packages names.
    pub fn package_names(&self) -> Vec<String> {
        self.category_names()
            .into_iter()
            .flat_map(|c| self.package_names_by_category(&c))
            .collect()
    }

    /// Returns the package IDs for the given package
    pub fn package_ids(&self, package: &str) -> Vec<PackageID> {
        paludis_repository_package_ids_canonical_form(self.ptr.to_owned(), package)
            .into_iter()
            .map(|n| {
                paludis_repository_package_id_from_canonical_form(self.ptr.to_owned(), package, &n)
            })
            .flatten()
            .map(|ptr| new_package_id(ptr))
            .collect()
    }

    fn string_medata_value(&self, key: &str) -> Option<String> {
        if !self.metadata_exist(key) {
            None
        } else {
            unsafe { Some(self.metadata_key_unchecked(key).value_str()) }
        }
    }

    /// The format metadata, if non-null, holds the repository's format. [`Repository`] implementations should not return [`None`] here, but clients should still check.
    pub fn format(&self) -> Option<String> {
        self.string_medata_value("format")
    }

    /// The location metadata, if non-null, holds the file or directory containing the repository's data, the format of which depends on the value of format_key.
    pub fn location(&self) -> Option<PathBuf> {
        Some(PathBuf::from(self.string_medata_value("location")?))
    }

    /// The repository short description.
    pub fn summary(&self) -> Option<String> {
        self.string_medata_value("summary")
    }

    /// List of repositories this repository depend on.
    pub fn master_repositories_names(&self) -> Vec<String> {
        let key = "master_repository";
        if !self.metadata_exist(key) {
            Vec::new()
        } else {
            unsafe {
                self.metadata_key_unchecked(key)
                    .value_str()
                    .split('\n')
                    .map(|s| String::from(s))
                    .collect()
            }
        }
    }

    /// Test if a metadata is stored at the key provided, in this repository.
    pub fn metadata_exist(&self, metadata: &str) -> bool {
        paludis_repository_metadata_exist(self.ptr.to_owned(), metadata)
    }

    /// List of the metadata keys of this repository.
    pub fn metadata_names(&self) -> Vec<String> {
        paludis_repository_metadata_names(self.ptr.to_owned())
    }

    /// Get metadata key by name
    pub fn metadata_key(&self, metadata: &str) -> Option<MetadataKey> {
        if !self.metadata_exist(metadata) {
            None
        } else {
            Some(new_metadata_key(paludis_repository_metadata_key(
                self.ptr.to_owned(),
                metadata,
            )))
        }
    }

    /// Same as metadata_key, but the metadata key existancy is not checked: could segfault.
    pub unsafe fn metadata_key_unchecked(&self, metadata: &str) -> MetadataKey {
        new_metadata_key(paludis_repository_metadata_key(
            self.ptr.to_owned(),
            metadata,
        ))
    }
}
