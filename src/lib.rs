
use std::path::Path;

use log::{debug, info, warn, error};
use serde::{Deserialize, Serialize};
use cfg_if::cfg_if;

/// Configuration for a system library.
///
/// This is generally configured in `build.rs` for a `*-sys` crate,
/// features are selected using crate features. Configuration values
/// may be overrided with `[metadata.LIBRARY]` in the project `Cargo.toml`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Library base name
    name: String,

    /// External library
    library: Option<Library>,

    /// Local source (in crate)
    source_dir: Option<SourceDir>,

    /// Git source (outside crate)
    git_repo: Option<GitRepo>,
}

/// External (system) library
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub version: Option<String>,
}

/// Local directory source for library
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDir {
    pub path: String,
}

/// Git repository source or library
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GitRepo {
    pub repo: String,
    pub reference: Option<String>,
}

/// Information required to build or link a library
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryInfo {

}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CcOptions {

}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AutotoolsOptions {

}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CMakeOptions {

}


impl Config {
    fn new(name: &str, version: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            library: Some(Library {
                name: name.to_string(),
                version: version.map(|v| v.to_string()),
            }),
            source_dir: None,
            git_repo: None,
        }
    }

    pub fn source_dir(mut self, dir: &str) -> Self {
        self.source_dir = Some(SourceDir{path: dir.to_string()});
        self
    }

    pub fn git_repo(mut self, repo: &str, reference: Option<&str>) -> Self {
        self.git_repo = Some(GitRepo{
            repo: repo.to_string(),
            reference: reference.map(|v| v.to_string()),
        });
        self
    }

    pub fn cc(mut self, opts: CcOptions) -> Self {
        unimplemented!()
    }

    pub fn build(&mut self) -> Result<(), ()> {

        // Default to using system libraries if enabled
        let mut lib = match &self.library {
            Some(s) if cfg!(features = "use_pkgconfig") => {
                Self::find_pkgconfig(s)
            },
            Some(s) if cfg!(features = "use_vcpkg") => {
                Self::find_vcpkg(s)
            },
            _ => None
        };

        // Check we found a useful system library
        if let Some(l) = lib {
            // TODO: use this and exit
            return Ok(())
        }

        // Use sources if enabled
        let source_dir = match (&self.source_dir, &self.git_repo) {
            (Some(d), _) if cfg!(features = "source_dir") => {
                Self::find_source_dir(d)
            },
            (_, Some(g)) if cfg!(features = "git") => {
                Self::find_source_git(g)
            },
            _ => {
                info!("No local or git sources available");
                None
            }
        };

        // Check we found a source
        let s = match source_dir {
            Some(s) => s,
            None => {
                error!("No sources available, ensure a source is configured in `build.rs` and enabled with `source_git` or `source_dir` configuration flags");
                return Err(())
            }
        };

        // TODO: use the provided compiler


        // TODO: copy or re-generate bindings

        unimplemented!()
    }

    /// Find source in an existing directory
    fn find_source_dir(dir: &SourceDir) -> Option<String> {
        // Check source dir exists
        if Path::new(&dir.path).exists() {
            info!("Located source directory {}", dir.path);
            Some(dir.path.clone())
        } else {
            None
        }
    }

    /// Find source using git
    fn find_source_git(repo: &GitRepo) -> Option<String> {
        unimplemented!()
    }
    
    /// Find existing library using pkg config
    fn find_pkgconfig(lib: &Library) -> Option<LibraryInfo> {
        let mut c = &mut pkg_config::Config::new();

        if let Some(v) = &lib.version {
            c.atleast_version(v);
        }

        if cfg!(feature = "static") {
            c.statik(true);
        }

        match c.probe(&lib.name) {
            Ok(l) => {
                info!("pkg-config found library {:?}", l);
                // TODO: propagate library info
                unimplemented!();
            },
            Err(e) => {
                error!("pkg-config failed to find library {}", lib.name);
                None
            }
        }
    }

    /// Find existing library using vcpkg
    fn find_vcpkg(lib: &Library) -> Option<LibraryInfo> {
        match vcpkg::find_package(&lib.name){
            Ok(l) => {
                info!("vcpkg found library {:?}", l);
                // TODO: propagate library info
                unimplemented!();
            },
            Err(e) => {
                error!("vcpkg failed to find library {}", lib.name);
                None
            }
        }
    }


    /// Build using CC crate
    fn build_cc(&self, opts: CcOptions) -> Result<(), ()> {
        unimplemented!()
    }

    /// Build using autotools crate
    fn build_autotools(&self, opts: AutotoolsOptions) -> Result<(), ()> {
        unimplemented!()
    }

    /// Build using cmake
    fn build_cmake(&self, opts: CMakeOptions) -> Result<(), ()> {
        unimplemented!()
    }
}