use ift::rfc::{
    Rfc6890Entry,
    RfcEntry::{
        self,
        Rfc6890,
    },
};
use ipnet::IpNet;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RfcInfo {
    pub output: RfcEntry,
}

pub fn parse_tables(tables: &str) -> Vec<RfcInfo> {
    let re = Regex::new(r"(?m)\+----+\+----+\+[^|]+\|\s*Attribute\s*\|\s*Value\s*\|[\r\n]+([^+]+\+----+\+----+\+)[\r\n]+([^+]+)\+----+\+----+\+").unwrap();

    re.captures_iter(tables)
        .map(|cap| {
            let head = &cap[1];
            let table = &cap[2];
            parse_table(head, table)
        })
        .collect()
}

fn parse_table(head: &str, table: &str) -> RfcInfo {
    let v: Vec<usize> = head.match_indices('+').map(|tup| tup.0).collect();
    assert_eq!(
        3,
        v.len(),
        "expected that the regex only matched 3 '+' signs"
    );

    let mut output = HashMap::new();
    for row in table.split('\n') {
        if !row.trim().is_empty() {
            assert_eq!(
                v[2] + 1,
                row.len(),
                "expected that each row contains the correct number. [{}], [{}]",
                row,
                table
            );

            let k = row[v[0] + 1..v[1] - 1].trim();
            let v = row[v[1] + 1..v[2] - 1].trim();
            output.insert(k.to_owned(), v.to_owned());
        }
    }

    //println!("{:?}", output);
    let entry = Rfc6890Entry {
        address_block: parse_address_block(&output, "Address Block"),
        name: output["Name"].to_owned(),
        rfc: output["RFC"].to_owned(),
        allocation_date: output["Allocation Date"].to_owned(),
        termination_date: output["Termination Date"].to_owned(),
        source: parse_bool(&output, "Source"),
        destination: parse_bool(&output, "Destination"),
        forwardable: parse_bool(&output, "Forwardable"),
        global: parse_bool(&output, "Global"),
        reserved_by_protocol: parse_bool(&output, "Reserved-by-Protocol"),
    };
    //println!("{:?}", entry);

    RfcInfo {
        output: Rfc6890(entry),
    }
}

fn remove_footnote<'a, 'b>(map: &'a HashMap<String, String>, key: &'b str) -> &'a str {
    let v = &map[key];
    v.split('[')
        .next()
        .unwrap_or_else(|| panic!("split should not return nil. using key[{}]", key))
        .trim()
}

fn parse_address_block(map: &HashMap<String, String>, key: &str) -> IpNet {
    let v = remove_footnote(map, key);
    v.to_lowercase()
        .parse()
        .unwrap_or_else(|_| panic!("unable to parse [{}] as ip net", v))
}

fn parse_bool(map: &HashMap<String, String>, key: &str) -> bool {
    let v = remove_footnote(map, key).to_lowercase();
    match v.as_str() {
        "n/a" => false,
        _ => v
            .parse()
            .unwrap_or_else(|_| panic!("unable to parse [{}] as bool", v)),
    }
}

#[cfg(test)]
mod tests {
    use crate::rfc_parser::{
        parse_table,
        parse_tables,
    };
    use ift::rfc::RfcEntry::Rfc6890;

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
        let Rfc6890(r) = parse_table(head, table).output;
        assert!(r.forwardable);
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
                    | Address Block        | 10.0.0.0/8 [2]|
                    | Name                 | Private-Use   |
                    | RFC                  | [RFC1918]     |
                    | Allocation Date      | February 1996 |
                    | Termination Date     | N/A           |
                    | Source               | True          |
                    | Destination          | True          |
                    | Forwardable          | True [1]      |
                    | Global               | N/A           |
                    | Reserved-by-Protocol | False         |
                    +----------------------+---------------+

        ";
        let out = parse_tables(tables);
        assert_eq!(2, out.len());
        let Rfc6890(ref r) = out[1].output;
        assert_eq!(false, r.global);
    }

}
