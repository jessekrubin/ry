pub(crate) trait PyGlobPatterns {
    fn patterns_ref(&self) -> &Vec<String>;
}

/// trait for thingy that implements PyGlobPatterns
pub(crate) trait PyGlobPatternsString: PyGlobPatterns {
    fn patterns_string(&self) -> String {
        let inner_str = self
            .patterns_ref()
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{inner_str}]")
    }
}
