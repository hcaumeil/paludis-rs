#![allow(unused)]
#![recursion_limit = "2048"]

//! Rust bindings to the [paludis](https://paludis.exherbo.org/) multi-format package manager C++ library.
//!
//! It is advised to read paludis [documentation](https://paludis.exherbo.org/api/cplusplus/index.html) first.
//!
//! Interfacing C++ can be complicated : some features in this lib are possible with "hacks".
//! To fully understand what you are doing, read carefully the documentation and the higher level functions code of this crate (functions that you can see in the documentation).

//! ## Exemple
//!
//! ```rust
//! use paludis_rs::Environment;
//!
//! // Collecting summary (if defined) for each repository in the local environnement
//! fn main() {
//!     let env = Environment::default();
//!     let mut summaries: Vec<(String, String)> = Vec::new();
//!
//!     env.repositories(|r| {
//!         summaries.push((r.name(), r.summary().unwrap_or(String::from("No summary"))))
//!     });
//!
//!     for e in summaries {
//!         println!("{}:\n{}", e.0, e.1);
//!     }
//! }
//! ```

mod bindings;
mod environment;
mod metadata;
mod packageid;
mod repository;
mod test;

pub use bindings::extract_host_from_url;
pub use environment::Environment;
pub use metadata::MetadataKey;
pub use metadata::MetadataKeyType;
pub use packageid::PackageID;
pub use packageid::VersionSpec;
pub use repository::Repository;
