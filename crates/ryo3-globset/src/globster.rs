use std::path::Path;

use globset::{Glob, GlobMatcher, GlobSet, GlobSetBuilder};
use pyo3::PyResult;
use pyo3::exceptions::PyValueError;

#[derive(Clone, Debug)]
pub(crate) enum GlobsterStrategy {
    Empty,
    SinglePositive(GlobMatcher),
    SingleNegative(GlobMatcher),
    MultiPositive(GlobSet),
    MultiNegative(GlobSet),
    Ordered(Vec<GlobsterStrategy>),
}

#[derive(Clone, Debug)]
pub struct Globster {
    pub(crate) strategy: GlobsterStrategy,
    pub patterns: Vec<String>,
    pub length: usize,
}

impl Globster {
    pub(crate) fn from_globset(patterns: Vec<String>, globset: GlobSet) -> Self {
        let length = patterns.len();
        Self {
            strategy: if length == 0 {
                GlobsterStrategy::Empty
            } else {
                GlobsterStrategy::MultiPositive(globset)
            },
            patterns,
            length,
        }
    }

    pub(crate) fn from_positive_glob(pattern: String, glob: Glob) -> Self {
        Self {
            strategy: GlobsterStrategy::SinglePositive(glob.compile_matcher()),
            patterns: vec![pattern],
            length: 1,
        }
    }

    pub(crate) fn is_match_path(&self, path: &Path) -> bool {
        self.strategy.is_match_path(path)
    }

    pub(crate) fn is_match_str(&self, path: &str) -> bool {
        self.strategy.is_match_str(path)
    }
}

impl GlobsterStrategy {
    pub(crate) fn from_globs(negative: bool, mut globs: Vec<Glob>) -> PyResult<Self> {
        if globs.is_empty() {
            return Ok(Self::Empty);
        }
        if globs.len() == 1 {
            let matcher = globs.remove(0).compile_matcher();
            return Ok(if negative {
                Self::SingleNegative(matcher)
            } else {
                Self::SinglePositive(matcher)
            });
        }

        let mut globset_builder = GlobSetBuilder::new();
        for glob in globs {
            globset_builder.add(glob);
        }
        let globset = globset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;

        Ok(if negative {
            Self::MultiNegative(globset)
        } else {
            Self::MultiPositive(globset)
        })
    }

    fn is_match_path(&self, path: &Path) -> bool {
        match self {
            Self::Empty => false,
            Self::SinglePositive(matcher) => matcher.is_match(path),
            Self::SingleNegative(matcher) => !matcher.is_match(path),
            Self::MultiPositive(globset) => globset.is_match(path),
            Self::MultiNegative(globset) => !globset.is_match(path),
            Self::Ordered(strategies) => ordered_is_match_path(strategies, path),
        }
    }

    fn is_match_str(&self, path: &str) -> bool {
        match self {
            Self::Empty => false,
            Self::SinglePositive(matcher) => matcher.is_match(path),
            Self::SingleNegative(matcher) => !matcher.is_match(path),
            Self::MultiPositive(globset) => globset.is_match(path),
            Self::MultiNegative(globset) => !globset.is_match(path),
            Self::Ordered(strategies) => ordered_is_match_str(strategies, path),
        }
    }

    fn raw_match_path(&self, path: &Path) -> bool {
        match self {
            Self::Empty => false,
            Self::SinglePositive(matcher) | Self::SingleNegative(matcher) => matcher.is_match(path),
            Self::MultiPositive(globset) | Self::MultiNegative(globset) => globset.is_match(path),
            Self::Ordered(strategies) => ordered_is_match_path(strategies, path),
        }
    }

    fn raw_match_str(&self, path: &str) -> bool {
        match self {
            Self::Empty => false,
            Self::SinglePositive(matcher) | Self::SingleNegative(matcher) => matcher.is_match(path),
            Self::MultiPositive(globset) | Self::MultiNegative(globset) => globset.is_match(path),
            Self::Ordered(strategies) => ordered_is_match_str(strategies, path),
        }
    }

    fn is_positive(&self) -> bool {
        matches!(
            self,
            Self::SinglePositive(_) | Self::MultiPositive(_) | Self::Ordered(_)
        )
    }
}

fn ordered_is_match_path(strategies: &[GlobsterStrategy], path: &Path) -> bool {
    let mut matched = false;
    for strategy in strategies {
        if strategy.raw_match_path(path) {
            matched = strategy.is_positive();
        }
    }
    matched
}

fn ordered_is_match_str(strategies: &[GlobsterStrategy], path: &str) -> bool {
    let mut matched = false;
    for strategy in strategies {
        if strategy.raw_match_str(path) {
            matched = strategy.is_positive();
        }
    }
    matched
}
