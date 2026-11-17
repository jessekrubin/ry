#![doc = include_str!("../README.md")]

mod ry_walkdir_entry;
mod ry_walkdir_iter;

use std::path::Path;

use pyo3::prelude::*;
use ryo3_core::sync::RyMutex;
use ryo3_core::types::PathLike;
use ryo3_globset::{GlobsterLike, RyGlobster};

pub use crate::ry_walkdir_entry::RyWalkDirEntry;
pub use crate::ry_walkdir_iter::RyWalkDirIter;

#[expect(clippy::struct_excessive_bools)]
struct WalkDirOptions {
    files: bool,
    dirs: bool,
    contents_first: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    follow_links: bool,
    follow_root_links: bool,
    same_file_system: bool,
    sort_by_file_name: bool,
}

impl Default for WalkDirOptions {
    fn default() -> Self {
        Self {
            files: true,
            dirs: true,
            contents_first: false,
            min_depth: 0,
            max_depth: None,
            follow_links: false,
            follow_root_links: true,
            same_file_system: false,
            sort_by_file_name: false,
        }
    }
}

impl WalkDirOptions {
    fn build_walkdir<T: AsRef<Path>>(&self, path: T) -> ::walkdir::WalkDir {
        let mut wd = ::walkdir::WalkDir::new(path)
            .contents_first(self.contents_first)
            .follow_links(self.follow_links)
            .follow_root_links(self.follow_root_links)
            .same_file_system(self.same_file_system)
            .min_depth(self.min_depth);

        if let Some(max_depth) = self.max_depth {
            wd = wd.max_depth(max_depth);
        }
        if self.sort_by_file_name {
            wd = wd.sort_by(|a, b| a.file_name().cmp(b.file_name()));
        }
        wd
    }

    fn build_iter<T: AsRef<Path>>(
        &self,
        path: T,
    ) -> impl Iterator<Item = ::walkdir::DirEntry> + use<T> {
        let wd = self.build_walkdir(path.as_ref());
        let dirs = self.dirs;
        let files = self.files;
        let predicate = move |entry: &::walkdir::DirEntry| {
            let ftype = entry.file_type();
            (ftype.is_file() && files) || (ftype.is_dir() && dirs)
        };

        wd.into_iter()
            .filter_map(Result::ok)
            .filter(move |entry: &::walkdir::DirEntry| predicate(entry))
    }
}

#[expect(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
#[pyfunction]
#[pyo3(
    signature = (
        path = None,
        /,
        *,
        files = true,
        dirs = true,
        contents_first = false,
        min_depth = 0,
        max_depth = None,
        follow_links = false,
        follow_root_links = true,
        same_file_system = false,
        sort_by_file_name = false,
        glob = None,
    )
)]
pub fn walkdir(
    path: Option<PathLike>,
    files: bool,                // true
    dirs: bool,                 // true
    contents_first: bool,       // false
    min_depth: usize,           // default 0
    max_depth: Option<usize>,   // default None
    follow_links: bool,         // default false
    follow_root_links: bool,    // default true
    same_file_system: bool,     // default false
    sort_by_file_name: bool,    // default false
    glob: Option<GlobsterLike>, // default None
) -> PyResult<RyWalkDirIter> {
    let walk_globster = match glob {
        Some(g) => Some(RyGlobster::try_from(&g)?),
        None => None,
    };
    let opts = WalkDirOptions {
        files,
        dirs,
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        follow_root_links,
        same_file_system,
        sort_by_file_name,
    };
    let wd_iter = opts.build_iter(path.unwrap_or_else(|| PathLike::Str(String::from("."))));
    let final_iter = if let Some(gs) = walk_globster {
        Box::new(wd_iter.filter(move |entry| gs.is_match(entry.path())))
            as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    } else {
        Box::new(wd_iter) as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    };
    Ok(RyWalkDirIter(RyMutex::new(final_iter)))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyWalkDirIter>()?;
    m.add_class::<RyWalkDirEntry>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
