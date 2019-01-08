use pest_derive::*;

#[derive(Parser)]
#[grammar = "ift.pest"]
pub struct IfTParser;
