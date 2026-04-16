use crate::configuration::Configuration;

/// Context carried during IR generation.
#[allow(dead_code)]
pub struct Context<'a> {
    pub config: &'a Configuration,
    pub source: &'a str,
    /// Track nesting depth: 0 = top-level, > 0 = inside field/group.
    pub depth: usize,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Configuration, source: &'a str) -> Self {
        Self {
            config,
            source,
            depth: 0,
        }
    }

    #[allow(dead_code)]
    pub fn is_top_level(&self) -> bool {
        self.depth == 0
    }
}
