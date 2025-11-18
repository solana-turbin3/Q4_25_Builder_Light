use crate::report::report::*;
use crate::report::knowledge_base::{MissingInitIfNeededFinding, Finding, RULE_MISSING_INIT_IF_NEEDED};
use crate::state::NormalizedAccountStruct;

pub fn rules_missing_init_if_needed(s: &NormalizedAccountStruct, r: &mut Report) {
    // println!("Here");
    for field in &s.fields {
        if field.contains_attr("associated_token") {
            // println!("Probably Token");
            if field.has_bool_attribute("init") {
                // println!("Bug here");
                r.add(Finding::MissingInitIfNeeded(MissingInitIfNeededFinding {
                    rule: &RULE_MISSING_INIT_IF_NEEDED,
                    account: field.name.clone(),
                    line: field.line,
                    context: s.name.clone()
                    
                }));
            }
        }
    }
}