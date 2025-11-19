// Submodules
pub mod rule_init_if_needed;
pub mod rule_missing_fn_arg;
pub mod rule_wrong_space_assignment;
pub mod rule_missing_account_verification;
pub mod visit;

// Re-export all rules for easier access
pub use rule_init_if_needed::*;
pub use rule_missing_fn_arg::*;
pub use rule_wrong_space_assignment::*;
pub use rule_missing_account_verification::*;
pub use visit::*;
