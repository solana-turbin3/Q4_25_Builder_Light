use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Severity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct RuleMetadata {
    pub code: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub severity: Severity,
    pub recommendation: &'static str,
    pub additional_links: Option<&'static str>,
}

/* ────────────────────────────────────────────────────────────────
   A001 — Use init_if_needed Instead of init
   ──────────────────────────────────────────────────────────────── */
pub static RULE_MISSING_INIT_IF_NEEDED: RuleMetadata = RuleMetadata {
    code: "H-001",
    title: "Use init_if_needed Instead of init",
    description:
        "When initializing an Associated Token Account (ATA), using `init` will unconditionally \
         fail if the ATA already exists. This introduces fragility in programs, since ATAs are \
         often created by wallets, prior interactions, or other programs. A failed ATA creation \
         results in a panic and causes the entire transaction to revert. Using `init_if_needed` \
         ensures the instruction is idempotent and safe, regardless of the ATA’s existence state. \
         This pattern aligns with modern Solana security expectations and significantly reduces \
         user-triggered transaction failures.",
    severity: Severity::High,
    recommendation:
        "Replace `init` with `init_if_needed` for ATA initialization. This ensures safe, \
         idempotent behavior even when an ATA already exists. Always prefer `init_if_needed` \
         unless you have a strict requirement that the account must be newly created.",
    additional_links: Some(
        "https://www.anchor-lang.com/docs/associated-accounts\n\
         https://solana-labs.github.io/solana-program-library/associated-token-account"
    ),
};

/* ────────────────────────────────────────────────────────────────
   A002 — Incorrect Space Assignment
   ──────────────────────────────────────────────────────────────── */
pub static RULE_WRONG_SPACE_ASSIGNMENT: RuleMetadata = RuleMetadata {
    code: "M-001",
    title: "Incorrect Space Assignment",
    description:
        "A mismatch between a struct’s declared `space` value and its actual serialized size \
         leads to account truncation or unsafe overwrites. If `space` is too small, random data \
         corruption or panic conditions may occur when Anchor attempts to serialize state. If too \
         large, excessive rent is charged. Accurate space calculation is essential for safe and \
         predictable on-chain storage behavior.",
    severity: Severity::Medium,
    recommendation:
        "Recalculate the exact byte-length of the struct and update the `space` attribute to \
         reflect the correct size. Ensure that every field is included in the calculation and \
         account for all padding, discriminators, and aggregate data types.",
    additional_links: Some(
        "https://book.anchor-lang.com/chapter_3/space.html\n\
         https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html"
    ),
};

/* ────────────────────────────────────────────────────────────────
   A003 — Missing Required Argument in Instruction
   ──────────────────────────────────────────────────────────────── */
pub static RULE_MISSING_REQUIRED_INSTRUCTION_ARGUMENT: RuleMetadata = RuleMetadata {
    code: "M-002",
    title: "Missing Required Argument in Instruction",
    description:
        "The context struct declares one or more `#[instruction(...)]` arguments that the \
         function must receive, but the function signature omits them. This creates \
         inconsistencies between expected and actual instruction parameters, potentially \
         leading to logic bugs, incorrect PDA derivations, or unintended behavior when \
         serializing input data. A function that does not accept all of its declared \
         instruction parameters cannot rely on deterministic inputs.",
    severity: Severity::Medium,
    recommendation:
        "Ensure the function signature includes **all** arguments declared inside \
         `#[instruction(...)]`. The context definition and function parameters must match \
         exactly so callers provide the expected values.",
    additional_links: Some(
        "https://www.anchor-lang.com/docs/the-accounts-attribute"
    ),
};

/* ────────────────────────────────────────────────────────────────
   A004 — Possible Division by Zero
   ──────────────────────────────────────────────────────────────── */
pub static RULE_POSSIBLE_DIVISION_BY_ZERO: RuleMetadata = RuleMetadata {
    code: "L-001",
    title: "Possible Division by Zero",
    description:
        "This code performs a division using a runtime-controlled variable. If the \
         divisor evaluates to zero, the program will panic and revert the entire transaction. \
         Division involving user-provided values, account data, or unvalidated computations \
         requires explicit zero-checks. On-chain financial logic such as fee splits, reward \
         calculations, or escrow payments frequently exhibit this weakness when insufficient \
         input validation is applied.",
    severity: Severity::Low,
    recommendation:
        "Before performing the division, add a strict check ensuring the divisor is non-zero. \
         Reject or sanitize invalid inputs. For performance and safety, perform this validation \
         as early as possible in the function logic, and document expected input constraints.",
    additional_links: Some(
        "https://solana.com/developers/guides/programs/errors\n\
         https://book.anchor-lang.com/chapter_2/errors.html"
    ),
};



#[derive(Debug, Clone)]
pub struct MissingInitIfNeededFinding {
    pub rule: &'static RuleMetadata,
    pub line: usize,
    pub account: String,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct WrongSpaceAssignmentFinding {
    pub rule: &'static RuleMetadata,
    pub line: usize,
    pub account: String,
    pub expected: usize,
    pub actual: String,
}

#[derive(Debug, Clone)]
pub struct MissingRequiredInstructionArgumentFinding {
    pub rule: &'static RuleMetadata,
    pub line: usize,
    pub fn_name: String,
    pub required_arg_name: String,
    pub required_arg_type: String,
}

#[derive(Debug, Clone)]
pub struct PossibleDivisionByZeroFinding {
    pub rule: &'static RuleMetadata,
    pub line: usize,
    pub fn_name: String,
    pub divisor: String,
}

/// Unified enum so the report system can store all findings
#[derive(Debug, Clone)]
pub enum Finding {
    MissingInitIfNeeded(MissingInitIfNeededFinding),
    WrongSpaceAssignment(WrongSpaceAssignmentFinding),
    MissingRequiredInstructionArgument(MissingRequiredInstructionArgumentFinding),
    PossibleDivisionByZero(PossibleDivisionByZeroFinding)
}
