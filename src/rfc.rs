use ipnet::IpNet;
use std::net::IpAddr;

mod rfc6890_entries;

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

pub struct WithRfc6890 {
    pub entries: Vec<Rfc6890Entry>,
}
impl WithRfc6890 {
    pub fn create() -> WithRfc6890 { rfc6890_entries::entries() }

    /// RFC6890 https://tools.ietf.org/rfc/rfc6890.txt
    ///
    /// Forwardable - A boolean value indicating whether a router may
    ///      forward an IP datagram whose destination address is drawn from the
    ///      allocated special-purpose address block between external
    ///      interfaces.
    ///
    /// ```
    /// use ift::rfc::WithRfc6890;
    /// let rfc = WithRfc6890::create();
    ///
    /// assert_eq!(true, rfc.is_forwardable(&"192.168.1.100".parse().unwrap()), "intranet ip");
    /// assert_eq!(false, rfc.is_forwardable(&"169.254.169.254".parse().unwrap()), "aws metadata service");
    /// assert_eq!(true, rfc.is_forwardable(&"172.217.9.142".parse().unwrap()), "a google ip");
    /// assert_eq!(true, rfc.is_forwardable(&"2001:4860:4860::8844".parse().unwrap()), "a google ipv6 address");
    /// ```
    ///
    pub fn is_forwardable(&self, ip: &IpAddr) -> bool {
        let most_specific = self.find_most_specific(ip);

        if let Some(entry) = most_specific {
            entry.forwardable
        } else {
            // if it is not found than it is forwardable
            true
        }
    }

    /// RFC6890 https://tools.ietf.org/rfc/rfc6890.txt
    ///
    /// Global - A boolean value indicating whether an IP datagram whose
    ///      destination address is drawn from the allocated special-purpose
    ///      address block is forwardable beyond a specified administrative
    ///      domain.
    ///
    /// ```
    /// use ift::rfc::WithRfc6890;
    /// let rfc = WithRfc6890::create();
    ///
    /// assert_eq!(false, rfc.is_global(&"192.168.1.100".parse().unwrap()), "intranet ip");
    /// assert_eq!(true, rfc.is_global(&"192.88.99.20".parse().unwrap()), "internet ip");
    /// assert_eq!(false, rfc.is_global(&"169.254.169.254".parse().unwrap()), "aws metadata service");
    /// assert_eq!(true, rfc.is_global(&"172.217.9.142".parse().unwrap()), "a google ip");
    /// assert_eq!(true, rfc.is_global(&"64:ff9b::255.255.255.255".parse().unwrap()), "ipv6");
    /// ```
    ///
    pub fn is_global(&self, ip: &IpAddr) -> bool {
        let most_specific = self.find_most_specific(ip);

        if let Some(entry) = most_specific {
            entry.global
        } else {
            true
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

#[cfg(test)]
mod tests {
    use crate::rfc::WithRfc6890;
    use ipnet::IpNet;

    // check that 192 because there are multiple definitions and
    // we want to make sure that it picks the most specific one
    #[test]
    fn is_forwardable_or_not_192() {
        let all: IpNet = "192.0.0.0/24".parse().unwrap();
        let specific: IpNet = "192.0.0.0/29".parse().unwrap();
        let rfc = WithRfc6890::create();
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

    // check 192 for global
    #[test]
    fn is_global_192() {
        let all: IpNet = "192.88.99.0/24".parse().unwrap();
        let rfc = WithRfc6890::create();
        for ip_addr in all.hosts() {
            assert!(rfc.is_global(&ip_addr), "failure on ip {}", ip_addr)
        }
    }
}