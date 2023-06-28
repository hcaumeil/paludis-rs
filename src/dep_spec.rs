use cxx::SharedPtr;
use std::fmt::Debug;

use super::bindings::paludis_dependencieslabel_text;
use super::bindings::paludis_dependencyspectree_all_len;
use super::bindings::paludis_dependencyspectree_all_val;
use super::bindings::paludis_dependencyspectree_labels_len;
use super::bindings::paludis_dependencyspectree_labels_val;
use super::bindings::paludis_dependencyspectree_namedset;
use super::bindings::paludis_dependencyspectree_package;
use super::bindings::paludis_dependencyspectree_type;
use super::bindings::paludis_packagedepspecdata_fullname;

#[derive(Debug)]
pub enum DependencySpecTree {
    None,
    NamedSet(String),
    Labels(Vec<DependenciesLabel>),
    Package(PackageDepSpecData),
    All(Vec<DependencySpecTree>),
}

pub fn new_dependencyspectree(ptr: SharedPtr<u64>) -> DependencySpecTree {
    let t = paludis_dependencyspectree_type(ptr.clone());
    match t {
        0 => DependencySpecTree::NamedSet(paludis_dependencyspectree_namedset(ptr)),
        1 => {
            let mut res = Vec::new();
            for i in 0..paludis_dependencyspectree_labels_len(ptr.clone()) {
                res.push(new_dependencies_label(
                    paludis_dependencyspectree_labels_val(ptr.clone(), i),
                ));
            }
            DependencySpecTree::Labels(res)
        }
        2 => DependencySpecTree::Package(new_packagedepspec_data(
            paludis_dependencyspectree_package(ptr),
        )),
        6 => {
            let mut res = Vec::new();
            for i in 0..paludis_dependencyspectree_all_len(ptr.clone()) {
                res.push(new_dependencyspectree(paludis_dependencyspectree_all_val(
                    ptr.clone(),
                    i,
                )));
            }
            DependencySpecTree::All(res)
        }
        a => {
            println!("type {a}");
            DependencySpecTree::None
        }
    }
}

pub struct DependenciesLabel {
    ptr: SharedPtr<u64>,
}

pub fn new_dependencies_label(ptr: SharedPtr<u64>) -> DependenciesLabel {
    DependenciesLabel { ptr }
}

impl Debug for DependenciesLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text());
        Ok(())
    }
}

impl DependenciesLabel {
    pub fn text(&self) -> String {
        paludis_dependencieslabel_text(self.ptr.to_owned())
    }
}

pub struct PackageDepSpecData {
    ptr: SharedPtr<u64>,
}

pub fn new_packagedepspec_data(ptr: SharedPtr<u64>) -> PackageDepSpecData {
    PackageDepSpecData { ptr }
}

impl Debug for PackageDepSpecData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.full_name());
        Ok(())
    }
}

impl PackageDepSpecData {
    pub fn full_name(&self) -> String {
        paludis_packagedepspecdata_fullname(self.ptr.to_owned())
    }
}
