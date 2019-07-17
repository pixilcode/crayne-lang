//! Utilities needed throughout the project

/// The type of input for `nom`
/// 
/// `Input` can be used as input for `nom` parsers
pub type Input<'a> = &'a str;