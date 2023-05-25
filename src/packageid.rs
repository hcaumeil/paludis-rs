use cxx::SharedPtr;

use super::bindings::paludis_packageid_metadata_exist;
use super::bindings::paludis_packageid_metadata_names;
use super::bindings::paludis_packageid_name;
use super::bindings::paludis_packageid_short_description;
use super::bindings::paludis_packageid_version;
use super::bindings::paludis_versionspec_compare;
use super::bindings::paludis_versionspec_eq;
use super::bindings::paludis_versionspec_is_scm;
use super::bindings::paludis_versionspec_valid;

use super::MetadataKey;
use super::Repository;

/// Represents a version number (for example, 1.2.3b-r1).
pub struct VersionSpec(String);

impl VersionSpec {
    /// Is this an -scm package, or something pretending to be one?
    // FIXME : Error handeling : cant tell if aspell-pt_BR-20131030.12.0 is scm (version validity)
    pub fn is_scm(&self) -> bool {
        paludis_versionspec_is_scm(self.0.as_str())
    }

    /// Test if the VersionSpec paludis object is constructible from this version spec
    pub fn is_valid(&self) -> bool {
        paludis_versionspec_valid(self.0.as_str())
    }
}

impl Into<String> for VersionSpec {
    fn into(self) -> String {
        self.0
    }
}

impl ToString for VersionSpec {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl PartialEq for VersionSpec {
    fn eq(&self, other: &Self) -> bool {
        paludis_versionspec_eq(self.0.as_str(), other.0.as_str())
    }
}

impl Eq for VersionSpec {}

impl PartialOrd for VersionSpec {
    // FIXME : Error handling in paludis_versionspec_compare is not so good : package version like elinks-scm and elinks-0.12pre6-r1 are difficult to compare (version validity)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = paludis_versionspec_compare(self.0.as_str(), other.0.as_str());

        if cmp == 0 {
            return Some(std::cmp::Ordering::Equal);
        } else if cmp.is_negative() {
            return Some(std::cmp::Ordering::Less);
        } else if cmp.is_positive() {
            return Some(std::cmp::Ordering::Greater);
        } else {
            // Classic quantum bug
            return None;
        }
    }
}

impl Ord for VersionSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| std::cmp::Ordering::Equal)
    }
}

/// Represents a unique package version in a particular [`Repository`].
///
/// All PackageID instances have some basic identification data:
/// - A name
/// - A version
/// - An owning repository
/// It should be noted that the above together are not sufficient to uniquely identify a package.
/// Some repositories support multiple slots per version of a package, and some repositories support additional affixes that prevent a package from being uniquely identifiable merely by the above.
///
/// PackageID instances also have:
/// - A collection of [`MetadataKey`]. Some of these keys have a particular specific role in certain places.
/// - A collection (often empty) of masks. A masked package cannot be installed.
///
/// A PackageID instance may support certain actions, which are represented via an Action subclass instance.
pub struct PackageID {
    ptr: SharedPtr<u64>,
}

impl PackageID {
    pub fn name(&self) -> String {
        paludis_packageid_name(self.ptr.to_owned())
    }

    pub fn version(&self) -> VersionSpec {
        VersionSpec(paludis_packageid_version(self.ptr.to_owned()))
    }

    pub fn short_description(&self) -> String {
        paludis_packageid_short_description(&self.ptr)
    }

    /// Test if a metadata is stored at the key provided, in this repository.
    pub fn metadata_exist(&self, metadata: &str) -> bool {
        return paludis_packageid_metadata_exist(self.ptr.to_owned(), metadata);
    }

    /// Test if a metadata is stored at the key provided, in this repository.
    pub fn metadata_names(&self) -> Vec<String> {
        return paludis_packageid_metadata_names(self.ptr.to_owned());
    }
}

pub fn new_package_id(ptr: SharedPtr<u64>) -> PackageID {
    PackageID { ptr }
}
