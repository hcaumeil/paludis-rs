use super::bindings::paludis_environment_fetch_repository;
use super::bindings::paludis_environment_has_repository_named;
use super::bindings::paludis_environment_new;
use super::bindings::paludis_environment_repositories_names;

use super::repository::new_repository;
use super::Repository;

use cxx::SharedPtr;

/// Represents a working environment, which contains an available packages database and provides various methods for querying package visibility and options.
/// Holds a number of [`Repository`] instances.
pub struct Environment {
    ptr: SharedPtr<u64>,
}

impl Environment {
    /// Create an environment from the given spec.
    /// A spec consisits of class:suffix both of which may be omitted.
    /// class is the environment class, e.g. paludis or portage, suffix is the configuration directory suffix.
    /// If the configuration directory is empty, it will exit the program and throws "paludis::paludis_environment::PaludisConfigNoDirectoryError"
    pub fn new(spec: &str) -> Self {
        Environment {
            ptr: paludis_environment_new(spec),
        }
    }

    /// Iterate over repositories the safe way.
    pub fn repositories<F>(&self, mut f: F)
    where
        F: FnMut(&Repository),
    {
        for name in self.repositories_names() {
            let r = self.fetch_repository(&name).unwrap();
            f(&r);
        }
    }

    /// Return the list of the names of the repositories in the environment.
    pub fn repositories_names(&self) -> Vec<String> {
        paludis_environment_repositories_names(&self.ptr)
    }

    /// Test if there is a repository named like provided in the environment.
    pub fn has_repository_named(&self, repository: &str) -> bool {
        paludis_environment_has_repository_named(&self.ptr, repository)
    }

    /// Test if the repository exist, and if yes, fetch it.
    /// Be carefull, because reposository depends on each others, having multiple [`Repository`] at the same time could be dangerous (seg fault incomming).
    /// Prefer [repositories](#method.repositories) iterator function.
    pub fn fetch_repository(&self, repository: &str) -> Option<Repository> {
        if !self.has_repository_named(repository) {
            None
        } else {
            Some(new_repository(paludis_environment_fetch_repository(
                &self.ptr, repository,
            )))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(":")
    }
}
