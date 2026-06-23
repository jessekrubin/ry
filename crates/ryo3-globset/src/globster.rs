use std::path::Path;

use globset::{Candidate, GlobSet, GlobSetBuilder};

#[derive(Clone, Debug)]
pub(crate) struct Rule {
    pub(crate) negative: bool,
    pub(crate) globset: GlobSet,
}

impl From<GlobSet> for Rule {
    fn from(globset: GlobSet) -> Self {
        Self {
            negative: false,
            globset,
        }
    }
}

impl Rule {
    pub(crate) fn from_globs(
        negative: bool,
        globs: Vec<globset::Glob>,
    ) -> Result<Self, globset::Error> {
        let mut builder = GlobSetBuilder::new();
        for glob in globs {
            builder.add(glob);
        }
        Ok(Self {
            negative,
            globset: builder.build()?,
        })
    }

    fn is_match_candidate(&self, c: &Candidate<'_>) -> Option<bool> {
        self.globset.is_match_candidate(c).then_some(!self.negative)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum GlobsterMatcher {
    Empty,
    Set(GlobSet),
    Rules(Vec<Rule>),
}

impl GlobsterMatcher {
    fn is_match_candidate(&self, c: &Candidate<'_>) -> bool {
        match self {
            Self::Empty => false,
            Self::Set(gs) => gs.is_match_candidate(c),
            Self::Rules(rules) => rules.iter().fold(false, |matched, rule| {
                rule.is_match_candidate(c).unwrap_or(matched)
            }),
        }
    }

    fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        self.is_match_candidate(&Candidate::new(path.as_ref()))
    }
}

#[derive(Clone, Debug)]
pub struct Globster {
    pub(crate) matcher: GlobsterMatcher,
    pub patterns: Vec<String>,
}

impl Globster {
    pub fn is_match_candidate(&self, c: &Candidate<'_>) -> bool {
        self.matcher.is_match_candidate(c)
    }

    pub fn is_match_path(&self, path: &Path) -> bool {
        self.matcher.is_match(path)
    }

    pub fn is_match_str(&self, s: &str) -> bool {
        self.matcher.is_match(s)
    }

    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    pub(crate) fn from_globset(patterns: Vec<String>, globset: GlobSet) -> Self {
        let matcher = if patterns.is_empty() {
            GlobsterMatcher::Empty
        } else {
            GlobsterMatcher::Set(globset)
        };
        Self { matcher, patterns }
    }

    pub(crate) fn from_positive_glob(pattern: String, glob: &globset::Glob) -> Self {
        let mut builder = GlobSetBuilder::new();
        builder.add(glob.clone());
        let globset = builder.build().expect("single valid glob always builds");
        Self {
            matcher: GlobsterMatcher::Set(globset),
            patterns: vec![pattern],
        }
    }
}
