use super::bindings::paludis_environment_create_sync_output_manager;
use super::bindings::paludis_environment_fetch_repository;
use super::bindings::paludis_environment_has_repository_named;
use super::bindings::paludis_environment_new;
use super::bindings::paludis_environment_repositories_names;

use super::output_manager::*;
use super::repository::new_repository;
use super::Repository;

use cxx::SharedPtr;

pub enum OuputExclusivity {
    // Run in the background, produce no output
    Background,
    // Other things may be running at the same time
    WithOthers,
    // We are the only thing running
    Exclusive,
}

impl Into<u8> for OuputExclusivity {
    fn into(self) -> u8 {
        match self {
            OuputExclusivity::Background => 0,
            OuputExclusivity::WithOthers => 1,
            OuputExclusivity::Exclusive => 2,
        }
    }
}

pub struct CreateOutputManagerSyncInfo {
    pub repository: String,
    pub ouput_exculivity: OuputExclusivity,
    pub summary: bool,
}

impl CreateOutputManagerSyncInfo {
    pub fn no_output(repository: &str) -> Self {
        CreateOutputManagerSyncInfo {
            repository: repository.to_owned(),
            ouput_exculivity: OuputExclusivity::Background,
            summary: false,
        }
    }
}

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

    /// Create an output manager to see repository sync infos.
    /// Need to be executed with root privilege if it output logs.
    pub fn create_sync_output_manager(
        &self,
        options: CreateOutputManagerSyncInfo,
    ) -> Option<OutputManager> {
        paludis_environment_create_sync_output_manager(
            &self.ptr,
            &options.repository,
            options.ouput_exculivity.into(),
            options.summary,
        )
        .map(|oe| new_output_manager(oe))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(":")
    }
}
