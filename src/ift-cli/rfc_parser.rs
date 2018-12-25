#[derive(Debug)]
pub struct RfcInfo<'a> {
    output: &'a str,
}

pub fn parse_tables(tables: &str) -> Vec<RfcInfo> {
    vec![RfcInfo{
        output: "something"
    }]
}