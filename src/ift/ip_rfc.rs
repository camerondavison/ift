use crate::rfc6890_entries;
use ipnet::IpNet;
use std::net::IpAddr;

#[derive(Debug)]
pub enum RfcEntry {
    Rfc6890(Rfc6890Entry),
}

#[derive(Debug)]
pub struct Rfc6890Entry {
    pub address_block: IpNet,
    pub name: String,
    pub rfc: String,
    pub allocation_date: String,
    pub termination_date: String,
    pub source: bool,
    pub destination: bool,
    pub forwardable: bool,
    pub global: bool,
    pub reserved_by_protocol: bool,
}

impl Rfc6890Entry {
    pub fn as_code(&self) -> String {
        format!(
            "\
Rfc6890Entry {{
    address_block: \"{}\".parse().unwrap(),
    name: \"{}\".to_owned(),
    rfc: \"{}\".to_owned(),
    allocation_date: \"{}\".to_owned(),
    termination_date: \"{}\".to_owned(),
    source: {},
    destination: {},
    forwardable: {},
    global: {},
    reserved_by_protocol: {}
}}",
            self.address_block,
            escape_quotes(&self.name),
            escape_quotes(&self.rfc),
            escape_quotes(&self.allocation_date),
            escape_quotes(&self.termination_date),
            self.source,
            self.destination,
            self.forwardable,
            self.global,
            self.reserved_by_protocol
        )
    }
}

pub struct Rfc6890 {
    pub entries: Vec<Rfc6890Entry>,
}
impl Rfc6890 {
    pub fn create() -> Rfc6890 {
        rfc6890_entries::entries()
    }

    pub fn is_forwardable(&self, ip: &IpAddr) -> bool {
        let most_specific = self.find_most_specific(ip);

        if let Some(entry) = most_specific {
            entry.forwardable
        } else {
            false
        }
    }

    pub fn is_global(&self, ip: &IpAddr) -> bool {
        let most_specific = self.find_most_specific(ip);

        if let Some(entry) = most_specific {
            entry.global
        } else {
            false
        }
    }

    fn find_most_specific(&self, ip: &IpAddr) -> Option<&Rfc6890Entry> {
        let mut most_specific: Option<&Rfc6890Entry> = None;
        for cur in &self.entries {
            if cur.address_block.contains(ip) {
                if let Some(existing) = most_specific {
                    if existing.address_block.contains(&cur.address_block) {
                        most_specific = Some(cur);
                    }
                } else {
                    most_specific = Some(cur);
                }
            }
        }
        most_specific
    }
}

fn escape_quotes(s: &str) -> String {
    s.replace('"', r#"\""#)
}

#[cfg(test)]
mod tests {
    use crate::ip_rfc::Rfc6890;
    use ipnet::IpNet;

    #[test]
    fn get_interface_ip_192() {
        let all: IpNet = "192.0.0.0/24".parse().unwrap();
        let specific: IpNet = "192.0.0.0/29".parse().unwrap();
        let rfc = Rfc6890::create();
        for ip_addr in all.hosts() {
            let is_forwardable = specific.contains(&ip_addr);
            assert_eq!(
                is_forwardable,
                rfc.is_forwardable(&ip_addr),
                "failure on ip {}",
                ip_addr
            )
        }
    }
}
