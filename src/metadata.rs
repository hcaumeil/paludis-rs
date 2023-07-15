use core::panic;

use cxx::SharedPtr;

use super::bindings::paludis_dependencyspectree_all_len;
use super::bindings::paludis_dependencyspectree_all_val;
use super::bindings::paludis_dependencyspectree_type;
use super::bindings::paludis_metadata_human_name;
use super::bindings::paludis_metadata_raw_name;
use super::bindings::paludis_metadata_type;
use super::bindings::paludis_metadata_type_str;
use super::bindings::paludis_metadata_value_dependencyspectree;
use super::bindings::paludis_metadata_value_str;
use super::bindings::paludis_metadata_value_string;
use super::bindings::paludis_metadata_value_type;

use super::dep_spec::new_dependencyspectree;

use super::DependencySpecTree;
use super::PackageID;
use super::Repository;

/// The significance of a MetadataKey to a user.
/// This is a hint to clients as to whether the key should be displayed when outputting information about a [`PackageID`] or [`Repository`].
#[derive(Debug, PartialEq, Eq)]
pub enum MetadataKeyType {
    /// A key of significant interest, to be shown early on.
    Significant,
    /// A normal key.
    Normal,
    /// Should only be shown if the user asks for author information.
    Author,
    /// Should only be shown if the user asks for dependencies.
    Dependencies,
    /// Should not usually be displayed to the user.
    Internal,
}

impl MetadataKeyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetadataKeyType::Significant => "Significant",
            MetadataKeyType::Normal => "Normal",
            MetadataKeyType::Author => "Author",
            MetadataKeyType::Dependencies => "Dependencies",
            MetadataKeyType::Internal => "Internal",
        }
    }
}

impl From<u8> for MetadataKeyType {
    fn from(n: u8) -> Self {
        match n {
            0 => MetadataKeyType::Significant,
            1 => MetadataKeyType::Normal,
            2 => MetadataKeyType::Author,
            3 => MetadataKeyType::Dependencies,
            4 => MetadataKeyType::Internal,
            _ => MetadataKeyType::Normal,
        }
    }
}

impl Into<&str> for MetadataKeyType {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl Into<&str> for &MetadataKeyType {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl ToString for MetadataKeyType {
    fn to_string(&self) -> String {
        let s: &str = self.into();
        s.into()
    }
}

/// A MetadataKey is a generic key that contains a particular piece of information about a PackageID or [`Repository`] instance.
///
/// A basic MetadataKey has:
/// - A raw name : This is in a repository-defined format designed to closely represent the internal name. For example, ebuilds and VDB IDs use raw names like 'DESCRIPTION' and 'KEYWORDS', whereas CRAN uses names like 'Title' and 'BundleDescription'. The raw name is unique in a PackageID or Repository.
/// - A human name : This is the name that should be used when outputting normally for a human to read.
/// - A [`MetadataKeyType`] : This is a hint to clients as to whether the key should be displayed when outputting information about a package ID or Repository.
pub struct MetadataKey {
    ptr: SharedPtr<u64>,
}

pub fn new_metadata_key(ptr: SharedPtr<u64>) -> MetadataKey {
    MetadataKey { ptr }
}

impl MetadataKey {
    pub fn human_name(&self) -> String {
        paludis_metadata_human_name(self.ptr.to_owned())
    }

    pub fn raw_name(&self) -> String {
        paludis_metadata_raw_name(self.ptr.to_owned())
    }

    pub fn key_type(&self) -> MetadataKeyType {
        paludis_metadata_type(self.ptr.to_owned()).into()
    }

    pub fn key_type_str(&self) -> String {
        paludis_metadata_type_str(self.ptr.to_owned())
    }

    /// Use paludis to get the value behind a MetadataKey, and use paludis to stringify it.
    pub fn value(&self) -> MetadataValue {
        match paludis_metadata_value_type(self.ptr.to_owned()) {
            0 => MetadataValue::String(paludis_metadata_value_string(self.ptr.to_owned())),
            12 => {
                // println!("{}", paludis_metadata_value_str(self.ptr.clone()));
                MetadataValue::DependencySpecTree(new_dependencyspectree(
                    paludis_metadata_value_dependencyspectree(self.ptr.to_owned()),
                ))
            }
            t => {
                // MetadataValue::String(t.to_string() + " " + &self.value_str())
                panic!("panic: unable to match MetadataValue type")
            }
        }
    }

    /// Use paludis to get the value behind a MetadataKey, and use paludis to stringify it.
    /// Prefer using other functions because you will have to do parsing and not all metadata can be directly transated to string.    
    pub fn value_str(&self) -> String {
        paludis_metadata_value_str(self.ptr.to_owned())
    }
}

#[derive(Debug)]
pub enum MetadataValue {
    String(String),
    Slot,
    DependencySpecTree(DependencySpecTree),
}
