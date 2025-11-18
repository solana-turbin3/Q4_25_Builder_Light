use crate::report::report::*;
use crate::report::knowledge_base::{MissingRequiredInstructionArgumentFinding, Finding, RULE_MISSING_REQUIRED_INSTRUCTION_ARGUMENT};
use crate::state::{NormalizedFunctionArgs, AccountInstructions};

pub fn rules_fn_missing_instruction_args(v: &Vec<AccountInstructions>, f: &Vec<NormalizedFunctionArgs>, r: &mut Report) {
    for field in v {
        let name = field.ctx_name.clone();
        for func in f {
            if func.context == name {
                println!("fn and ins match");
                for required_arg in &field.args {
                let exists = func.args.iter().any(|arg|arg.name == required_arg.name && arg.ty == required_arg.ty);

                if !exists {
                    // println!(
                    //     "Missing required instruction argument in function: `{}`: {}: {}",
                    //     func.name, required_arg.name, required_arg.ty);
                        r.add(Finding::MissingRequiredInstructionArgument(MissingRequiredInstructionArgumentFinding {
                            rule: &RULE_MISSING_REQUIRED_INSTRUCTION_ARGUMENT,
                            fn_name : func.name.clone(),
                            line: func.line,
                            required_arg_name: required_arg.name.clone(),
                            required_arg_type: required_arg.ty.clone()
                        }));
                }
            }
            }

        }
    }
}