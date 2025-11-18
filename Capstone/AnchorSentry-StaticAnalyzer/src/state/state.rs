use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Bool(bool),
    String(String),
}



#[derive(Debug, Clone)]
pub struct NormalizedAccountStruct {
    pub name: String,
    pub fields: Vec<NormalizedAccountField>,
    pub line: usize
}

//@note: Currently want to use this to represent derive(accounts), hopefully its enough
#[derive(Debug, Clone)]
pub struct NormalizedAccountField {
    //context account name i.e OpenEscrowAccount (ctx)
    pub context: String,
    //account name i.e signer
    pub name: String,

    // Type info
    //i.e Account, Signer, Program, UncheckedAcct
    pub base_type: String,
    // ["'info", "TokenAccount"] do i need to add 'info, everything uses info
    pub generic_args: Vec<String>,

    // Attributes
    //hashmap seems to be the closest to python dictionaries
    // account struct attribites i.e init_if_needed|payer (String), true|signer (AttributeValue)
    pub attributes: HashMap<String, AttributeValue>, // flattened attributes
    // Metadata, for line
    pub line: usize
}

impl NormalizedAccountField {
    pub fn contains_attr(&self, key: &str) -> bool {
        self.attributes.keys().any(|k| k.contains(key))
    }
    pub fn has_bool_attribute(&self, key: &str) -> bool {
        matches!(self.attributes.get(key), Some(AttributeValue::Bool(true)))
    }

    pub fn has_str_attribute(&self, key: &str) -> Option<&str> {
        if let Some(AttributeValue::String(s)) = self.attributes.get(key) {
            Some(s)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedFunctionArgs {
    pub name: String,
    pub context: String,
    pub args: Vec<FnArgs>,
    pub line: usize
}

#[derive(Debug, Clone)]
pub struct FnArgs {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct AccountInstructions {
    pub ctx_name: String,
    pub args: Vec<FnArgs>
}

#[derive(Debug, Clone)]
pub struct PossibleDivisionByZeroFindingCheckerInstance {
    pub function_name: String,
    pub divisor: String,
    pub line: usize
}

