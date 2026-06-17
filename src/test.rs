// Some common stuff for unit tests. The top level mod statement is gated in a #[cfg(test)]
// conditional, so we don't need to do that for everything in this module

macro_rules! data {
    ($name:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $name)
    };
}

use crate::environment::Environment;
use crate::pamext::PamHandleExt;
use anyhow::{Context, Result};
pub(crate) use data;
use std::cell::RefCell;
use std::collections::VecDeque;
use uzers::uid_t;
 use std::path::Path;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

pub(crate) const CERT_STR: &str = include_str!(data!("cert.pub"));

macro_rules! canned {
    ($name:ident) => {
        pub struct $name {
            answers: RefCell<VecDeque<&'static str>>,
        }

        impl $name {
            pub(crate) fn new(answers: Vec<&'static str>) -> Self {
                $name {
                    answers: RefCell::new(VecDeque::from(answers)),
                }
            }

            fn answer(&'_ self) -> anyhow::Result<String> {
                Ok(self.answers.borrow_mut().pop_front().unwrap().to_string())
            }
        }
    };
}

canned!(CannedEnv);
impl Environment for CannedEnv {
    fn get_homedir(&'_ self, _user: &str) -> Result<String> {
        self.answer()
    }

    fn get_hostname(&'_ self) -> Result<String> {
        self.answer()
    }

    fn get_fqdn(&'_ self) -> Result<String> {
        self.answer()
    }

    fn get_uid(&'_ self, _user: &str) -> anyhow::Result<uid_t> {
        panic!()
    }

    fn get_env(&'_ self, _: &str) -> Option<String> {
        self.answer().ok()
    }
}

canned!(CannedHandler);
impl PamHandleExt for CannedHandler {
    fn get_calling_user(&self) -> Result<String> {
        self.answer()
    }

    fn get_service(&self) -> Result<String> {
        self.answer()
    }
}

pub struct DummyEnv;

impl Environment for DummyEnv {
    fn get_homedir(&'_ self, _user: &str) -> Result<String> {
        panic!()
    }

    fn get_hostname(&'_ self) -> Result<String> {
        panic!()
    }

    fn get_fqdn(&'_ self) -> Result<String> {
        panic!()
    }

    fn get_uid(&'_ self, _user: &str) -> Result<uid_t> {
        panic!()
    }

    fn get_env(&'_ self, _: &str) -> Option<String> {
        panic!()
    }
}

pub struct DummyHandle;

impl PamHandleExt for DummyHandle {
    fn get_calling_user(&self) -> Result<String> {
        panic!()
    }

    fn get_service(&self) -> Result<String> {
        panic!()
    }
}

pub fn set_file_permissions(filename: &Path) -> Result<()> {
    // authorized_keys file should be owned by root:root and have 0600 permissions
    std::os::unix::fs::chown(filename, Some(0), Some(0)).with_context(|| format!("Tests tried to set file {:?} ownership to root:root but failed", filename));
    let perms = Permissions::from_mode(0o600);
    std::fs::set_permissions(filename, perms).with_context(|| format!("Tests tried to set permissions of file {:?} to 0o600 but failed", filename));
    Ok(())
}