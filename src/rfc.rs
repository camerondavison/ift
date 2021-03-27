//! to get specific information about rfcs used by the templates
use ipnet::IpNet;
use std::net::IpAddr;

mod rfc6890_entries;

/// Entry containing everything from the table specified in
/// [RFC6890](https://tools.ietf.org/rfc/rfc6890.txt)
///
#[derive(Debug)]
pub struct Rfc6890Entry {
    /// IpNet block
    pub address_block: IpNet,
    /// Name from RFC
    pub name: String,
    /// Original RFC
    pub rfc: String,
    /// Allocation Date
    pub allocation_date: String,
    /// If terminated by when
    pub termination_date: String,
    /// Is a source
    pub source: bool,
    /// Is a destination
    pub destination: bool,
    /// Is forwardable
    pub forwardable: bool,
    /// Is global
    pub global: bool,
    /// Is reserved
    pub reserved_by_protocol: bool,
}

/// Used to check IpAddr's against all the rfc 6890 entries and find the one that
/// matches the most specific definition
pub struct WithRfc6890 {
    /// vector of all of the available entries
    pub entries: Vec<Rfc6890Entry>,
}

impl WithRfc6890 {
    /// Build the WithRfc6890, by creating the list of Rfc6890Entry's
    pub fn create() -> WithRfc6890 {
        rfc6890_entries::entries()
    }

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
            // todo: maybe make this return true/false/empty (empty for not found?)
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
            // todo: maybe make this return true/false/empty (empty for not found?)
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
    use std::net::IpAddr;

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

    #[test]
    fn is_loopback() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        let rfc = WithRfc6890::create();
        assert_eq!(false, rfc.is_forwardable(&ip))
    }
}
