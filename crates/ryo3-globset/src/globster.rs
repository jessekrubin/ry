use std::path::Path;

use globset::{Candidate, Glob, GlobSet, GlobSetBuilder};
use pyo3::PyResult;
use pyo3::exceptions::PyValueError;

#[derive(Clone, Debug)]
pub(crate) enum GlobsterStrategyElement {
    Set(GlobSet),
    SetNeg(GlobSet),
}

#[derive(Clone, Debug)]
pub(crate) enum GlobsterStrategy {
    Empty,
    One(GlobsterStrategyElement),
    Ignore(Vec<GlobsterStrategyElement>),
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
                GlobsterStrategy::One(GlobsterStrategyElement::Set(globset))
            },
            patterns,
            length,
        }
    }

    pub(crate) fn from_positive_glob(pattern: String, glob: &Glob) -> Self {
        let strategy = GlobsterStrategy::from_globs(false, vec![glob.clone()])
            .expect("wenodis: globset from a valid glob dont fail");
        Self {
            strategy,
            patterns: vec![pattern],
            length: 1,
        }
    }

    pub fn is_match_candidate(&self, path: &Candidate<'_>) -> bool {
        self.strategy.is_match_candidate(path)
    }

    pub(crate) fn is_match_path(&self, path: &Path) -> bool {
        self.strategy.is_match(path)
    }

    pub(crate) fn is_match_str(&self, path: &str) -> bool {
        self.strategy.is_match(path)
    }
}

impl GlobsterStrategy {
    pub(crate) fn from_globs(negative: bool, globs: Vec<Glob>) -> PyResult<Self> {
        if globs.is_empty() {
            return Ok(Self::Empty);
        }
        GlobsterStrategyElement::from_globs(negative, globs).map(Self::One)
    }

    fn is_match_candidate(&self, path: &Candidate<'_>) -> bool {
        match self {
            Self::Empty => false,
            Self::One(strategy) => strategy.is_match_candidate(path),
            Self::Ignore(strategies) => strategies.iter().fold(false, |matched, strategy| {
                strategy.match_effect_candidate(path).unwrap_or(matched)
            }),
        }
    }

    fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        let c = Candidate::new(path.as_ref());
        self.is_match_candidate(&c)
    }
}

impl GlobsterStrategyElement {
    pub(crate) fn from_globs(negative: bool, globs: Vec<Glob>) -> PyResult<Self> {
        if globs.is_empty() {
            return Err(PyValueError::new_err(
                "Cannot build globster strategy from empty globs",
            ));
        }

        let mut globset_builder = GlobSetBuilder::new();
        for glob in globs {
            globset_builder.add(glob);
        }
        let globset = globset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;

        Ok(if negative {
            Self::SetNeg(globset)
        } else {
            Self::Set(globset)
        })
    }

    fn is_match_candidate(&self, path: &Candidate<'_>) -> bool {
        match self {
            Self::Set(globset) => globset.is_match_candidate(path),
            Self::SetNeg(globset) => !globset.is_match_candidate(path),
        }
    }

    fn match_effect_candidate(&self, path: &Candidate<'_>) -> Option<bool> {
        match self {
            Self::Set(globset) => globset.is_match_candidate(path).then_some(true),
            Self::SetNeg(globset) => globset.is_match_candidate(path).then_some(false),
        }
    }
}
