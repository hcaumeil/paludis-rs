use cpp::cpp;
use cxx::{CxxString, CxxVector, SharedPtr};
use std::ffi::CString;

cpp! {{
    #include <iostream>
    #include <string>
    #include <paludis/paludis.hh>
}}

/// Try to extract the hostname part of a URL.
/// Returns an empty string if nothing convincing can be found.
pub fn extract_host_from_url(s: &str) -> String {
    let arg = CString::new(s).unwrap();
    let ptr = arg.as_ptr();

    // let res = unsafe { std::mem::transmute::<*const u64, *const CxxString>(r) };
    unsafe {
        let temp = Box::from_raw(
            cpp!([ptr as "const char *"] -> *mut CxxString as "const std::string *" {
                std::string* res = new std::string (paludis::extract_host_from_url(ptr));
                return res;
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

// FIXME: if an environment don't exit the program abort
// maybe returning a null pointer in the catch section and make this func return an option ?
/// Equivalent of paludis EnvironmentFactory
pub fn paludis_environment_new(spec: &str) -> SharedPtr<u64> {
    let arg = CString::new(spec).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([ptr as "const char *"] -> SharedPtr<u64> as "std::shared_ptr<paludis::Environment>" {
            try {
              return paludis::EnvironmentFactory::get_instance()->create(ptr);
            } catch (const std::exception &e) {
              std::cerr << "error: \"" << e.what() << "\"\n";
              exit(1);
            }
        })
    }
}

pub fn paludis_environment_repositories_names(e: &SharedPtr<u64>) -> Vec<String> {
    let temp = unsafe {
        Box::from_raw(
            cpp!([e as "std::shared_ptr<paludis::Environment>*"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                for (const auto & r : (*e)->repositories())
                    res.push_back(r->name().value());
                return new std::vector<std::string>(res);
            }),
        )
    };
    (*temp)
        .into_iter()
        .map(|e| match (*e).to_str() {
            Ok(s) => Some(String::from(s)),
            Err(_) => None,
        })
        .flatten()
        .collect::<Vec<String>>()
}

pub fn paludis_environment_has_repository_named(e: &SharedPtr<u64>, repo: &str) -> bool {
    let arg = CString::new(repo).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([e as "std::shared_ptr<paludis::Environment>*", ptr as "const char *"] -> bool as "bool" {
            return (*e)->has_repository_named(paludis::RepositoryName(ptr));
        })
    }
}

pub fn paludis_environment_fetch_repository(e: &SharedPtr<u64>, repo: &str) -> SharedPtr<u64> {
    let arg = CString::new(repo).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([e as "std::shared_ptr<paludis::Environment>*", ptr as "const char *"] -> SharedPtr<u64> as "std::shared_ptr<paludis::Repository>" {
            return (*e)->fetch_repository(paludis::RepositoryName(ptr));
        })
    }
}

pub fn paludis_repository_name(r: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>"] -> *mut CxxString as "const std::string *" {
              return new std::string(r.get()->name().value());
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_repository_category_names(r: SharedPtr<u64>) -> Vec<String> {
    unsafe {
        let temp: Box<CxxVector<CxxString>> = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                for (const auto & category_name  : *(r.get()->category_names(paludis::RepositoryContentMayExcludes({}))))
                  res.push_back(std::string(stringify(category_name)));
                return new std::vector<std::string>(res);
            }),
        );

        (*temp)
            .into_iter()
            .map(|e| match (*e).to_str() {
                Ok(s) => Some(String::from(s)),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn paludis_repository_metadata_exist(r: SharedPtr<u64>, metadata: &str) -> bool {
    let arg = CString::new(metadata).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> bool as "bool" {
          std::set<std::shared_ptr<const paludis::MetadataKey>> keys(
              r->begin_metadata(), r->end_metadata());
          for (const auto &key : keys) {
            if (key->raw_name() == ptr) {
              return true;
            }
          }
          return false;
        })
    }
}

pub fn paludis_repository_metadata_names(r: SharedPtr<u64>) -> Vec<String> {
    unsafe {
        let temp: Box<CxxVector<CxxString>> = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                std::set<std::shared_ptr<const paludis::MetadataKey>> keys(
                    r->begin_metadata(), r->end_metadata());
                for (const auto &key : keys)
                  res.push_back(std::string(key->raw_name()));
                return new std::vector<std::string>(res);
            }),
        );

        (*temp)
            .into_iter()
            .map(|e| match (*e).to_str() {
                Ok(s) => Some(String::from(s)),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn paludis_repository_metadata_key(r: SharedPtr<u64>, metadata: &str) -> SharedPtr<u64> {
    let arg = CString::new(metadata).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> SharedPtr<u64> as "const std::shared_ptr<const paludis::MetadataKey>" {
            paludis::Repository::MetadataConstIterator it(r->find_metadata(ptr));
            return *it;
        })
    }
}

pub fn paludis_repository_package_names(r: SharedPtr<u64>, category: &str) -> Vec<String> {
    let arg = CString::new(category).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        let temp: Box<CxxVector<CxxString>> = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                try {
                  paludis::CategoryNamePart category(ptr);
                  std::shared_ptr<const paludis::QualifiedPackageNameSet> c(
                      r->package_names(category, {}));
                  for (const auto &p : *c)
                    res.push_back(std::string(stringify(p)));
                } catch (const std::exception &e) {}

                return new std::vector<std::string>(res);
            }),
        );

        (*temp)
            .into_iter()
            .map(|e| match (*e).to_str() {
                Ok(s) => Some(String::from(s)),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn paludis_repository_package_ids_canonical_form(
    r: SharedPtr<u64>,
    package: &str,
) -> Vec<String> {
    let arg = CString::new(package).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        let temp: Box<CxxVector<CxxString>> = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                try {
                  paludis::QualifiedPackageName q(ptr);
                  std::shared_ptr<const paludis::PackageIDSequence> c(r->package_ids(q, {}));
                  for (const auto &p : *c)
                    res.push_back(p.get()->canonical_form(paludis::idcf_full));
                } catch (const std::exception &e) {}

                return new std::vector<std::string>(res);
            }),
        );
        (*temp)
            .into_iter()
            .map(|e| match (*e).to_str() {
                Ok(s) => Some(String::from(s)),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn paludis_repository_package_id_from_canonical_form(
    r: SharedPtr<u64>,
    package: &str,
    canonical_form: &str,
) -> Option<SharedPtr<u64>> {
    let arg = CString::new(package).unwrap();
    let ptr = arg.as_ptr();

    let arg_can = CString::new(canonical_form).unwrap();
    let ptr_can = arg_can.as_ptr();

    let res: SharedPtr<u64> = unsafe {
        cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *", ptr_can as "const char *"] ->  SharedPtr<u64> as "std::shared_ptr<const paludis::PackageID>"{
            try {
              paludis::QualifiedPackageName q(ptr);
              std::shared_ptr<const paludis::PackageIDSequence> c(r->package_ids(q, {}));
              for (const auto &p : *c) {
                if (p.get()->canonical_form(paludis::idcf_full) == ptr_can)
                  return p;
              }
            } catch (const std::exception &e) {
            }
            return nullptr;
        })
    };

    if res.is_null() {
        return None;
    } else {
        return Some(res);
    }
}

pub fn paludis_packageid_name(p: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([p as "std::shared_ptr<paludis::PackageID>"] -> *mut CxxString as "const std::string *" {
                return new std::string(stringify(p.get()->name()));
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_packageid_version(p: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([p as "std::shared_ptr<paludis::PackageID>"] -> *mut CxxString as "const std::string *" {
                return new std::string(stringify(p.get()->version()));
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_packageid_metadata_exist(p: SharedPtr<u64>, metadata: &str) -> bool {
    let arg = CString::new(metadata).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([p as "std::shared_ptr<paludis::PackageID>", ptr as "const char *"] -> bool as "bool" {
          std::set<std::shared_ptr<const paludis::MetadataKey>> keys(
              p->begin_metadata(), p->end_metadata());
          for (const auto &key : keys) {
            if (key->raw_name() == ptr) {
              return true;
            }
          }
          return false;
        })
    }
}

pub fn paludis_packageid_metadata_names(p: SharedPtr<u64>) -> Vec<String> {
    unsafe {
        let temp: Box<CxxVector<CxxString>> = Box::from_raw(
            cpp!([p as "std::shared_ptr<paludis::PackageID>"] -> *mut CxxVector<CxxString> as "std::vector<std::string>*" {
                std::vector<std::string> res = {};
                std::set<std::shared_ptr<const paludis::MetadataKey>> keys(
                    p->begin_metadata(), p->end_metadata());
                for (const auto &key : keys)
                  res.push_back(std::string(key->raw_name()));
                return new std::vector<std::string>(res);
            }),
        );

        (*temp)
            .into_iter()
            .map(|e| match (*e).to_str() {
                Ok(s) => Some(String::from(s)),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn paludis_packageid_metadata_key(p: SharedPtr<u64>, metadata: &str) -> SharedPtr<u64> {
    let arg = CString::new(metadata).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([p as "std::shared_ptr<paludis::PackageID>", ptr as "const char *"] -> SharedPtr<u64> as "const std::shared_ptr<const paludis::MetadataKey>" {
            paludis::Repository::MetadataConstIterator it(p->find_metadata(ptr));
            return *it;
        })
    }
}

pub fn paludis_packageid_short_description(p: &SharedPtr<u64>) -> String {
    unsafe {
        let temp = cpp!([p as "std::shared_ptr<const paludis::PackageID>*"] -> *mut CxxString as "const std::string *" {
            return new std::string((*p)->short_description_key()->parse_value());
        });
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_versionspec_is_scm(v: &str) -> bool {
    let arg = CString::new(v).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([ptr as "const char *"] -> bool as "bool" {
            bool res = false;
            try {
              res = paludis::VersionSpec(std::string(ptr), {}).is_scm();
            } catch (const std::exception &e) {}
            return res;
        })
    }
}

pub fn paludis_versionspec_eq(v: &str, vo: &str) -> bool {
    let arg = CString::new(v).unwrap();
    let ptr = arg.as_ptr();

    let arg_o = CString::new(vo).unwrap();
    let ptr_o = arg_o.as_ptr();

    unsafe {
        cpp!([ptr as "const char *", ptr_o as "const char *"] -> bool as "bool" {
            bool res = false;
            try {
              res = paludis::VersionSpec(ptr, {}) == paludis::VersionSpec(ptr_o, {});
            } catch (const std::exception &e) {}

            return res;
        })
    }
}

pub fn paludis_versionspec_compare(v: &str, vo: &str) -> i8 {
    let arg = CString::new(v).unwrap();
    let ptr = arg.as_ptr();

    let arg_o = CString::new(vo).unwrap();
    let ptr_o = arg_o.as_ptr();

    unsafe {
        cpp!([ptr as "const char *", ptr_o as "const char *"] -> i8 as "int8_t" {
            int8_t res = 0;
            try {
              res = paludis::VersionSpec(ptr, {}).compare(paludis::VersionSpec(ptr_o, {}));
            } catch (const std::exception &e) {}

            return res;
        })
    }
}

// Non official function, just a nice "hack"
pub fn paludis_versionspec_valid(v: &str) -> bool {
    let arg = CString::new(v).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        cpp!([ptr as "const char *"] -> bool as "bool" {
            bool res = true;

            try {
              paludis::VersionSpec a = paludis::VersionSpec(ptr, {});
            } catch (const std::exception &e) {
              res = false;
            }

            return res;
        })
    }
}

pub fn paludis_metadata_human_name(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<const paludis::MetadataKey>"] -> *mut CxxString as "const std::string *"  {
                return new std::string(k.get()->human_name());
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_metadata_raw_name(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<const paludis::MetadataKey>"] -> *mut CxxString as "const std::string *"  {
                return new std::string(k.get()->raw_name());
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_metadata_type(k: SharedPtr<u64>) -> u8 {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::MetadataKey>"] -> u8 as "int8_t" {
            return k.get()->type();
        })
    }
}

pub fn paludis_metadata_type_str(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<paludis::MetadataKey>"] -> *mut CxxString as "const std::string *" {
                return new std::string(stringify(k->type()));
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_metadata_value_type(k: SharedPtr<u64>) -> u8 {
    unsafe {
        cpp!([k as "std::shared_ptr<paludis::MetadataKey>"] -> u8 as "uint8_t" {
            class MetadataVisitor {
            public:
              MetadataVisitor() {}

              uint8_t visit(const paludis::MetadataValueKey<std::string> &) {
                return 0;
              }

              uint8_t visit(const paludis::MetadataValueKey<paludis::Slot> &) {
                return 1;
              }

              uint8_t visit(const paludis::MetadataValueKey<long> &) { return 2; }

              uint8_t visit(const paludis::MetadataValueKey<bool> &) { return 3; }

              uint8_t visit( const paludis::MetadataValueKey<paludis::FSPath> &) {
                return 4;
              }

              uint8_t visit(const paludis::MetadataValueKey<
                           std::shared_ptr<const paludis::PackageID>> &) {
                return 5;
              }

              uint8_t visit(const paludis::MetadataTimeKey &) { return 6; }

              uint8_t visit(
                  const paludis::MetadataValueKey<std::shared_ptr<const paludis::Choices>>
                      &) {
                return 7;
              }

              uint8_t
              visit(const paludis::MetadataSpecTreeKey<paludis::PlainTextSpecTree> &) {
                return 8;
              }

              uint8_t visit(
                  const paludis::MetadataSpecTreeKey<paludis::RequiredUseSpecTree> &) {
                return 9;
              }

              uint8_t
              visit(const paludis::MetadataSpecTreeKey<paludis::LicenseSpecTree> &) {
                return 10;
              }

              uint8_t
              visit(const paludis::MetadataSpecTreeKey<paludis::SimpleURISpecTree> &) {
                return 11;
              }

              uint8_t visit(
                  const paludis::MetadataSpecTreeKey<paludis::DependencySpecTree> &) {
                return 12;
              }

              uint8_t
              visit(const paludis::MetadataSpecTreeKey<paludis::FetchableURISpecTree>
                        &) {
                return 13;
              }

              uint8_t
              visit(const paludis::MetadataCollectionKey<paludis::KeywordNameSet> &) {
                return 14;
              }

              uint8_t visit(
                  const paludis::MetadataCollectionKey<paludis::Set<std::string>> &) {
                return 15;
              }

              uint8_t visit(const paludis::MetadataCollectionKey<
                           paludis::Map<std::string, std::string>> &) {
                return 16;
              }

              uint8_t
              visit(const paludis::MetadataCollectionKey<paludis::Sequence<std::string>>
                        &) {
                return 17;
              }

              uint8_t
              visit(const paludis::MetadataCollectionKey<paludis::Maintainers> &) {
                return 18;
              }

              uint8_t
              visit(const paludis::MetadataCollectionKey<paludis::FSPathSequence> &) {
                return 19;
              }

              uint8_t visit(
                  const paludis::MetadataCollectionKey<paludis::PackageIDSequence> &) {
                return 20;
              }

              uint8_t visit(const paludis::MetadataSectionKey &) { return 21; }
            };

            MetadataVisitor v = MetadataVisitor();
            return k->accept_returning<uint8_t>(v);
        })
    }
}

pub fn paludis_metadata_value_str(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<paludis::MetadataKey>"] -> *mut CxxString as "const std::string *" {
                class MetadataVisitor {
                private:
                  std::string indent;

                public:
                  MetadataVisitor(const std::string &i = "") : indent(i) {}

                  static std::string
                  stringify_string_pair(const std::pair<const std::string, std::string> &s) {
                    if (s.first.empty())
                      return s.second;
                    else
                      return s.first + "=" + s.second;
                  }

                  static std::string
                  stringify_choice(const paludis::Choice &c) {
                      return c.raw_name();
                  }

                  std::string *visit(const paludis::MetadataValueKey<std::string> &key) {
                    return new std::string(indent + key.parse_value());
                  }

                  std::string *visit(const paludis::MetadataValueKey<paludis::Slot> &key) {
                    return new std::string(indent + key.parse_value().raw_value());
                  }

                  std::string *visit(const paludis::MetadataValueKey<long> &key) {

                    return new std::string(indent + std::to_string(key.parse_value()));
                  }

                  std::string *visit(const paludis::MetadataValueKey<bool> &key) {
                    return new std::string(indent + (key.parse_value() ? "true" : "false"));
                  }

                  std::string *visit(const paludis::MetadataValueKey<paludis::FSPath> &key) {
                    return new std::string(indent + stringify(key.parse_value()));
                  }

                  std::string *visit(const paludis::MetadataValueKey<
                                     std::shared_ptr<const paludis::PackageID>> &key) {
                    return new std::string(
                        indent + key.parse_value().get()->canonical_form(paludis::idcf_full));
                  }

                  std::string *visit(const paludis::MetadataTimeKey &key) {
                    return new std::string(indent +
                                           std::to_string(key.parse_value().seconds()));
                  }

                  std::string *visit(
                      const paludis::MetadataValueKey<std::shared_ptr<const paludis::Choices>>
                          &key) {
                    auto value(key.parse_value());
                    return new std::string(join(indirect_iterator(value->begin()),
                                                indirect_iterator(value->end()), "\n", stringify_choice));
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::PlainTextSpecTree> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *visit(
                      const paludis::MetadataSpecTreeKey<paludis::RequiredUseSpecTree> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::LicenseSpecTree> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::SimpleURISpecTree> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *visit(
                      const paludis::MetadataSpecTreeKey<paludis::DependencySpecTree> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::FetchableURISpecTree>
                            &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::KeywordNameSet> &key) {
                    return new std::string(
                        indent +
                        key.pretty_print_value(paludis::UnformattedPrettyPrinter(), {}));
                  }

                  std::string *visit(
                      const paludis::MetadataCollectionKey<paludis::Set<std::string>> &key) {
                    auto value(key.parse_value());
                    return new std::string(indent + join(value->begin(), value->end(), "\n"));
                  }

                  std::string *visit(const paludis::MetadataCollectionKey<
                                     paludis::Map<std::string, std::string>> &key) {
                    auto value(key.parse_value());
                    return new std::string(indent + join(value->begin(), value->end(), "\n",
                                                         stringify_string_pair));
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::Sequence<std::string>>
                            &key) {
                    auto value(key.parse_value());
                    return new std::string(indent + join(value->begin(), value->end(), "\n"));
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::Maintainers> &key) {
                    auto value(key.parse_value());
                    return new std::string(indent + join(value->begin(), value->end(), "\n"));
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::FSPathSequence> &key) {
                    auto value(key.parse_value());
                    return new std::string(indent + join(value->begin(), value->end(), "\n"));
                  }

                  std::string *visit(
                      const paludis::MetadataCollectionKey<paludis::PackageIDSequence> &key) {
                    auto value(key.parse_value());
                    return new std::string(join(indirect_iterator(value->begin()),
                                                indirect_iterator(value->end()), "\n"));
                  }

                  std::string *visit(const paludis::MetadataSectionKey &key) {
                    std::vector<std::string> res = {};

                    for (const auto &section_key : key.metadata()) {
                      MetadataVisitor v = MetadataVisitor(indent + "\t");
                      std::string *temp = section_key->accept_returning<std::string *>(v);
                      res.push_back(*temp);
                    }

                    return new std::string(indent +
                                            paludis::join(res.begin(), res.end(), "\n"));
                    }
                };
                MetadataVisitor v = MetadataVisitor();
                std::string *res = k->accept_returning<std::string *>(v);
                return res;
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_metadata_value_string(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<paludis::MetadataKey>"] -> *mut CxxString as "const std::string *" {
                class MetadataVisitor {
                public:
                  MetadataVisitor() {}

                  std::string *visit(const paludis::MetadataValueKey<std::string> &key) {
                    return new std::string(key.parse_value());
                  }


                  std::string *visit(const paludis::MetadataValueKey<paludis::Slot> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataValueKey<long> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataValueKey<bool> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataValueKey<paludis::FSPath> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataValueKey<
                                     std::shared_ptr<const paludis::PackageID>> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataTimeKey &) {
                      return new std::string("");
                  }

                  std::string *visit(
                      const paludis::MetadataValueKey<std::shared_ptr<const paludis::Choices>>
                          &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::PlainTextSpecTree> &) {
                      return new std::string("");
                  }

                  std::string *visit(
                      const paludis::MetadataSpecTreeKey<paludis::RequiredUseSpecTree> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::LicenseSpecTree> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::SimpleURISpecTree> &) {
                      return new std::string("");
                  }

                  std::string *visit(
                      const paludis::MetadataSpecTreeKey<paludis::DependencySpecTree> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataSpecTreeKey<paludis::FetchableURISpecTree>
                            &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::KeywordNameSet> &) {
                      return new std::string("");
                  }

                  std::string *visit(
                      const paludis::MetadataCollectionKey<paludis::Set<std::string>> &) {
                      return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataCollectionKey<
                                     paludis::Map<std::string, std::string>> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::Sequence<std::string>> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::Maintainers> &) {
                      return new std::string("");
                  }

                  std::string *
                  visit(const paludis::MetadataCollectionKey<paludis::FSPathSequence> &) {
                    return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataCollectionKey<paludis::PackageIDSequence> &) {
                    return new std::string("");
                  }

                  std::string *visit(const paludis::MetadataSectionKey &) {
                    return new std::string("");
                  }
                };

                MetadataVisitor v = MetadataVisitor();
                std::string *res = k->accept_returning<std::string *>(v);
                return res;
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_metadata_value_dependencyspectree(k: SharedPtr<u64>) -> SharedPtr<u64> {
    unsafe {
        cpp!([k as "std::shared_ptr<paludis::MetadataKey>"] -> SharedPtr<u64> as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>" {
            class MetadataVisitor {
            public:
              MetadataVisitor() {}

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<std::string> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }


              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<paludis::Slot> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<long> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<bool> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<paludis::FSPath> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataValueKey<
                                 std::shared_ptr<const paludis::PackageID>> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataTimeKey &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(
                  const paludis::MetadataValueKey<std::shared_ptr<const paludis::Choices>>
                      &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataSpecTreeKey<paludis::PlainTextSpecTree> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(
                  const paludis::MetadataSpecTreeKey<paludis::RequiredUseSpecTree> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataSpecTreeKey<paludis::LicenseSpecTree> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataSpecTreeKey<paludis::SimpleURISpecTree> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(
                  const paludis::MetadataSpecTreeKey<paludis::DependencySpecTree> &key) {
                  return key.parse_value()->top();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataSpecTreeKey<paludis::FetchableURISpecTree>
                        &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataCollectionKey<paludis::KeywordNameSet> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(
                  const paludis::MetadataCollectionKey<paludis::Set<std::string>> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataCollectionKey<
                                 paludis::Map<std::string, std::string>> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataCollectionKey<paludis::Sequence<std::string>> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataCollectionKey<paludis::Maintainers> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>
              visit(const paludis::MetadataCollectionKey<paludis::FSPathSequence> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataCollectionKey<paludis::PackageIDSequence> &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }

              std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> visit(const paludis::MetadataSectionKey &) {
                return std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>();
              }
            };

            MetadataVisitor v = MetadataVisitor();
            std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> res = k->accept_returning<std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>>(v);
            return res;
        })
    }
}

pub fn paludis_dependencyspectree_type(k: SharedPtr<u64>) -> u8 {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>"] -> u8 as "uint8_t" {
              class DependencySpecTreeVisitor {
              public:
                uint8_t t;
                DependencySpecTreeVisitor() {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                  t = 0;
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &) {
                    t = 1;
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &) {
                  t = 2;
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                    t = 3;
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                    t = 4;
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                    t = 5;
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &) {
                    t = 6;
                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor();
            k->accept(v);
            return v.t;
        })
    }
}

pub fn paludis_dependencyspectree_namedset(k: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>"] -> *mut CxxString as "const std::string *" {
                  class DependencySpecTreeVisitor {
                  public:
                    std::string res = "";
                    DependencySpecTreeVisitor() {}

                    void visit(const paludis::DependencySpecTree::NodeType<
                               paludis::NamedSetDepSpec>::Type &node) {
                        res = stringify(node.spec()->name());
                    }

                    void visit(const paludis::DependencySpecTree::NodeType<
                               paludis::DependenciesLabelsDepSpec>::Type &) {}

                    void visit(const paludis::DependencySpecTree::NodeType<
                               paludis::PackageDepSpec>::Type &) {
                    }

                    void visit(
                        const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                            &) {
                    }

                    void visit(const paludis::DependencySpecTree::NodeType<
                               paludis::ConditionalDepSpec>::Type &) {
                    }

                    void
                    visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                              &) {
                    }

                    void
                    visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                              &) {
                    }
                  };

                DependencySpecTreeVisitor v = DependencySpecTreeVisitor();
                k->accept(v);
                return new std::string(v.res);
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_dependencyspectree_labels_len(k: SharedPtr<u64>) -> u64 {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>"] -> u64 as "uint64_t" {
              class DependencySpecTreeVisitor {
              public:
                uint64_t res = 0;
                DependencySpecTreeVisitor() {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &node) {
                    for (auto n : *(node.spec()))
                        res++;
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &) {
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &) {

                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor();
            k->accept(v);
            return v.res;
        })
    }
}

pub fn paludis_dependencyspectree_labels_val(k: SharedPtr<u64>, i: u64) -> SharedPtr<u64> {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>", i as "uint64_t"] -> SharedPtr<u64> as "std::shared_ptr<const paludis::DependenciesLabel>" {
              class DependencySpecTreeVisitor {
              public:
                uint64_t val;
                std::shared_ptr<const paludis::DependenciesLabel> res = nullptr;
                DependencySpecTreeVisitor(const uint64_t v) : val(v) {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &node) {
                    uint64_t j = 0;
                    for (auto n : *(node.spec())) {
                        if (val == j) {
                            res = n;
                        }
                        j++;
                    }
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &) {
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &) {
                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor(i);
            k->accept(v);
            return v.res;
        })
    }
}

pub fn paludis_dependencyspectree_package(k: SharedPtr<u64>) -> SharedPtr<u64> {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>"] -> SharedPtr<u64> as "std::shared_ptr<const paludis::PackageDepSpecData>" {
              class DependencySpecTreeVisitor {
              public:
                std::shared_ptr<const paludis::PackageDepSpecData> res = nullptr;
                DependencySpecTreeVisitor() {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &) {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &node) {
                    res = node.spec()->data();
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &) {
                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor();
            k->accept(v);
            return v.res;
        })
    }
}

pub fn paludis_dependencyspectree_all_len(k: SharedPtr<u64>) -> u64 {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>"] -> u64 as "uint64_t" {
              class DependencySpecTreeVisitor {
              public:
                uint64_t res = 0;
                DependencySpecTreeVisitor() {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &) {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &) {
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &node) {

                    for (auto n : node)
                        res++;
                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor();
            k->accept(v);
            return v.res;
        })
    }
}

pub fn paludis_dependencyspectree_all_val(k: SharedPtr<u64>, i: u64) -> SharedPtr<u64> {
    unsafe {
        cpp!([k as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>", i as "uint64_t"] -> SharedPtr<u64> as "std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>>" {
              class DependencySpecTreeVisitor {
              public:
                uint64_t val;
                std::shared_ptr<const paludis::spec_tree_internals::BasicNode<paludis::DependencySpecTree>> res = nullptr;
                DependencySpecTreeVisitor(const uint64_t v) : val(v) {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::NamedSetDepSpec>::Type &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::DependenciesLabelsDepSpec>::Type &) {}

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::PackageDepSpec>::Type &) {
                }

                void visit(
                    const paludis::DependencySpecTree::NodeType<paludis::BlockDepSpec>::Type
                        &) {
                }

                void visit(const paludis::DependencySpecTree::NodeType<
                           paludis::ConditionalDepSpec>::Type &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AnyDepSpec>::Type
                          &) {
                }

                void
                visit(const paludis::DependencySpecTree::NodeType<paludis::AllDepSpec>::Type
                          &node) {
                    uint64_t j = 0;
                    for (auto n : node) {
                        if (val == j) {
                            res = n;
                        }
                        j++;
                    }
                }
              };

            DependencySpecTreeVisitor v = DependencySpecTreeVisitor(i);
            k->accept(v);
            return v.res;
        })
    }
}

pub fn paludis_packagedepspecdata_fullname(p: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([p as "std::shared_ptr<const paludis::PackageDepSpecData>"] -> *mut CxxString as "const std::string *" {
                return new std::string(stringify(*(p->package_ptr())));
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_dependencieslabel_text(l: SharedPtr<u64>) -> String {
    unsafe {
        let temp = Box::from_raw(
            cpp!([l as "std::shared_ptr<const paludis::DependenciesLabel>"] -> *mut CxxString as "const std::string *" {
                return new std::string(l->text());
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}
