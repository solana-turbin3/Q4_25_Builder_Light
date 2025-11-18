// Submodules
pub mod parser;
pub mod normalize;
pub mod rules;

// Re-export everything from submodules for easier access
pub use normalize::*;
pub use rules::*;
