use std::collections::HashMap;

#[derive(Debug)]
pub enum AttributeValue {
    Bool(bool),
    String(String),
}

pub struct NormalizedAccountStruct {
    pub name: String,
    pub fields: Vec<NormalizedAccountField>
}

//@note: Currently want to use this to represent derive(accounts), hopefully its enough
#[derive(Debug)]
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
    // pub span: Option<(usize, usize)>,
}

// //@note: Currently want to use this to represent derive(accounts), hopefully its enough
// pub struct NormalizedAccountField {
//     //context account name i.e OpenEscrowAccount (ctx)
//     pub context: String,
//     //account name i.e signer
//     pub name: String,

//     // Type info
//     //i.e Account, Signer, Program, UncheckedAcct
//     pub base_type: String,
//     // ["'info", "TokenAccount"] do i need to add 'info, everything uses info
//     pub generic_args: Vec<String>,
//     // is_mut: bool,

//     // Attributes
//     //hashmap seems to be the closest to python dictionaries
//     // account struct attribites i.e init_if_needed|payer (String), true|signer (AttributeValue)
//     pub attributes: HashMap<String, AttributeValue>, // flattened attributes
//     //probably check for duplicate seed names?
//     pub seeds: Option<Vec<String>>,
//     pub space: Option<String>,
//     //remember to read about bump errors
//     pub bump: Option<String>,

//     // Relationships
//     pub authority: Option<String>,
//     pub payer: Option<String>,
//     pub mint: Option<String>,
//     pub associated_to: Option<String>,

//     // Metadata
//     // span: Option<(usize, usize)>,
// }
