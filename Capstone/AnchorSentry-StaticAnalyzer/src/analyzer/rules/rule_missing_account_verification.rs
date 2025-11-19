use crate::report::report::*;
use crate::report::knowledge_base::{PossibleMissingAccountVerificationFinding, Finding, RULE_MISSING_ACCOUNT_VERIFICATION};
use crate::state::{NormalizedAccountStruct};

pub fn rules_missing_signer_check(s: &NormalizedAccountStruct, r: &mut Report) {
    for field in &s.fields {
        if field.base_type == "AccountInfo" || field.base_type == "UncheckedAccount" {
            // println!("{:?}", field);
            if field.has_bool_attribute("constraint") || field.has_bool_attribute("signer") {
                continue;
            }
            r.add(Finding::PossibleMissingAccountVerification(
            PossibleMissingAccountVerificationFinding {
                rule: &RULE_MISSING_ACCOUNT_VERIFICATION,
                line: field.line,
                account_name: field.name.clone(),
                field_type: field.base_type.clone(),
            }
        ));
        }
    }
}