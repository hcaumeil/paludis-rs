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

pub fn paludis_repository_metadata_type(r: SharedPtr<u64>, repo: &str) -> u8 {
    let arg = CString::new(repo).unwrap();
    let ptr = arg.as_ptr();

    // const int8_t
    // test5(std::shared_ptr<const paludis::MetadataKey> k) {
    //   // switch (k.get()->type()) {
    //   //   case paludis::MetadataKeyType::_normal : return 0;
    //   //   default : return 8;
    //   // }

    //   std::cout << stringify(k.get()->type()) << std::to_string((int8_t) k.get()->type());
    //   return k.get()->type();
    // }
    unsafe {
        cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> u8 as "int8_t" {
            paludis::Repository::MetadataConstIterator it(r->find_metadata(ptr));
            return (*it).get()->type();
        })
    }
}

pub fn paludis_repository_metadata_type_str(r: SharedPtr<u64>, repo: &str) -> String {
    let arg = CString::new(repo).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        let temp = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> *mut CxxString as "const std::string *" {
                paludis::Repository::MetadataConstIterator it(r->find_metadata(ptr));
                return new std::string(stringify((*it)->type()));
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
    }
}

pub fn paludis_repository_metadata_str(r: SharedPtr<u64>, metadata: &str) -> String {
    let arg = CString::new(metadata).unwrap();
    let ptr = arg.as_ptr();

    unsafe {
        let temp = Box::from_raw(
            cpp!([r as "std::shared_ptr<paludis::Repository>", ptr as "const char *"] -> *mut CxxString as "const std::string *" {
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
                paludis::Repository::MetadataConstIterator it(r->find_metadata(ptr));
                MetadataVisitor v = MetadataVisitor();
                std::string *res = (*it)->accept_returning<std::string *>(v);
                return res;
            }),
        );
        String::from((*temp).to_str().expect("str conversion goes wrong"))
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
