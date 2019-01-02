use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RfcInfo {
    pub output: HashMap<String, String>,
}

pub fn parse_tables(tables: &str) -> Vec<RfcInfo> {
    let re = Regex::new(r"(?m)\+----+\+----+\+[^|]+\|\s*Attribute\s*\|\s*Value\s*\|[\r\n]+([^+]+\+----+\+----+\+)[\r\n]+([^+]+)\+----+\+----+\+").unwrap();

    re.captures_iter(tables).map(|cap| {
        let head = &cap[1];
        let table = &cap[2];
        parse_table(head, table)
    }).collect()
}

fn parse_table(head: &str, table: &str) -> RfcInfo {
    let v: Vec<usize> = head.match_indices("+")
        .map(|tup| tup.0)
        .collect();
    assert_eq!(3, v.len(), "expected that the regex only matched 3 '+' signs");

    let mut info = RfcInfo { output: HashMap::new() };
    for row in table.split('\n') {
        if !row.trim().is_empty() {
            assert_eq!(v[2] + 1, row.len(), "expected that each row contains the correct number. [{}], [{}]", row, table);

            let k = row[v[0] + 1..v[1] - 1].trim();
            let v = row[v[1] + 1..v[2] - 1].trim();
            info.output.insert(k.to_owned(), v.to_owned());
        }
    }
    info
}

#[cfg(test)]
mod tests {
    use crate::rfc_parser::{parse_tables, parse_table};
    use std::collections::HashMap;

    #[test]
    fn test_parse_table() {
        let head = "                     +----------------------+----------------------+";
        let table = "                     | Address Block        | 100.64.0.0/10        |
                     | Name                 | Shared Address Space |
                     | RFC                  | [RFC6598]            |
                     | Allocation Date      | April 2012           |
                     | Termination Date     | N/A                  |
                     | Source               | True                 |
                     | Destination          | True                 |
                     | Forwardable          | True                 |
                     | Global               | False                |
                     | Reserved-by-Protocol | False                |";
        assert_eq!("True", parse_table(head, table).output["Forwardable"]);
    }

    #[test]
    fn test_parse_tables() {
        let tables = "

              +----------------------+----------------------------+
              | Attribute            | Value                      |
              +----------------------+----------------------------+
              | Address Block        | 0.0.0.0/8                  |
              | Name                 | \"This host on this network\"|
              | RFC                  | [RFC1122], Section 3.2.1.3 |
              | Allocation Date      | September 1981             |
              | Termination Date     | N/A                        |
              | Source               | True                       |
              | Destination          | False                      |
              | Forwardable          | False                      |
              | Global               | False                      |
              | Reserved-by-Protocol | True                       |
              +----------------------+----------------------------+

                    +----------------------+---------------+
                    | Attribute            | Value         |
                    +----------------------+---------------+
                    | Address Block        | 10.0.0.0/8    |
                    | Name                 | Private-Use   |
                    | RFC                  | [RFC1918]     |
                    | Allocation Date      | February 1996 |
                    | Termination Date     | N/A           |
                    | Source               | True          |
                    | Destination          | True          |
                    | Forwardable          | True          |
                    | Global               | False         |
                    | Reserved-by-Protocol | False         |
                    +----------------------+---------------+

        ";
        assert_eq!(2, parse_tables(tables).len())
    }

}