/*
https://tools.ietf.org/html/rfc6890
RFC 6890

The IPv4 and IPv6 Special-Purpose Address Registries maintain the
   following information regarding each entry:

   o  Address Block - A block of IPv4 or IPv6 addresses that has been
      registered for a special purpose.

   o  Name - A descriptive name for the special-purpose address block.

   o  RFC - The RFC through which the special-purpose address block was
      requested.

   o  Allocation Date - The date upon which the special-purpose address
      block was allocated.

   o  Termination Date - The date upon which the allocation is to be
      terminated.  This field is applicable for limited-use allocations
      only.

   o  Source - A boolean value indicating whether an address from the
      allocated special-purpose address block is valid when used as the
      source address of an IP datagram that transits two devices.

   o  Destination - A boolean value indicating whether an address from
      the allocated special-purpose address block is valid when used as
      the destination address of an IP datagram that transits two
      devices.

   o  Forwardable - A boolean value indicating whether a router may
      forward an IP datagram whose destination address is drawn from the
      allocated special-purpose address block between external
      interfaces.

   o  Global - A boolean value indicating whether an IP datagram whose
      destination address is drawn from the allocated special-purpose
      address block is forwardable beyond a specified administrative
      domain.

   o  Reserved-by-Protocol - A boolean value indicating whether the
      special-purpose address block is reserved by IP, itself.  This
      value is "TRUE" if the RFC that created the special-purpose
      address block requires all compliant IP implementations to behave
      in a special way when processing packets either to or from
      addresses contained by the address block.

   If the value of "Destination" is FALSE, the values of "Forwardable"
   and "Global" must also be false.
*/
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
    entries: Vec<Rfc6890Entry>,
}

impl Rfc6890 {
    pub fn create() -> Rfc6890 {
        Rfc6890 {
            entries: vec![
                Rfc6890Entry {
                    address_block: "0.0.0.0/8".parse().unwrap(),
                    name: "\"This host on this network".to_owned(),
                    rfc: "[RFC1122], Section 3.2.1.3".to_owned(),
                    allocation_date: "September 1981".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "10.0.0.0/8".parse().unwrap(),
                    name: "Private-Use".to_owned(),
                    rfc: "[RFC1918]".to_owned(),
                    allocation_date: "February 1996".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "100.64.0.0/10".parse().unwrap(),
                    name: "Shared Address Space".to_owned(),
                    rfc: "[RFC6598]".to_owned(),
                    allocation_date: "April 2012".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "127.0.0.0/8".parse().unwrap(),
                    name: "Loopback".to_owned(),
                    rfc: "[RFC1122], Section 3.2.1.3".to_owned(),
                    allocation_date: "September 1981".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "169.254.0.0/16".parse().unwrap(),
                    name: "Link Local".to_owned(),
                    rfc: "[RFC3927]".to_owned(),
                    allocation_date: "May 2005".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "172.16.0.0/12".parse().unwrap(),
                    name: "Private-Use".to_owned(),
                    rfc: "[RFC1918]".to_owned(),
                    allocation_date: "February 1996".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "192.0.0.0/24".parse().unwrap(),
                    name: "IETF Protocol Assignments".to_owned(),
                    rfc: "Section 2.1 of this document".to_owned(),
                    allocation_date: "January 2010".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "192.0.0.0/29".parse().unwrap(),
                    name: "DS-Lite".to_owned(),
                    rfc: "[RFC6333]".to_owned(),
                    allocation_date: "June 2011".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "192.0.2.0/24".parse().unwrap(),
                    name: "Documentation (TEST-NET-1)".to_owned(),
                    rfc: "[RFC5737]".to_owned(),
                    allocation_date: "January 2010".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "192.88.99.0/24".parse().unwrap(),
                    name: "6to4 Relay Anycast".to_owned(),
                    rfc: "[RFC3068]".to_owned(),
                    allocation_date: "June 2001".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: true,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "192.168.0.0/16".parse().unwrap(),
                    name: "Private-Use".to_owned(),
                    rfc: "[RFC1918]".to_owned(),
                    allocation_date: "February 1996".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "198.18.0.0/15".parse().unwrap(),
                    name: "Benchmarking".to_owned(),
                    rfc: "[RFC2544]".to_owned(),
                    allocation_date: "March 1999".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "198.51.100.0/24".parse().unwrap(),
                    name: "Documentation (TEST-NET-2)".to_owned(),
                    rfc: "[RFC5737]".to_owned(),
                    allocation_date: "January 2010".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "203.0.113.0/24".parse().unwrap(),
                    name: "Documentation (TEST-NET-3)".to_owned(),
                    rfc: "[RFC5737]".to_owned(),
                    allocation_date: "January 2010".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "240.0.0.0/4".parse().unwrap(),
                    name: "Reserved".to_owned(),
                    rfc: "[RFC1112], Section 4".to_owned(),
                    allocation_date: "August 1989".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "255.255.255.255/32".parse().unwrap(),
                    name: "Limited Broadcast".to_owned(),
                    rfc: "[RFC0919], Section 7".to_owned(),
                    allocation_date: "October 1984".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: true,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "::1/128".parse().unwrap(),
                    name: "Loopback Address".to_owned(),
                    rfc: "[RFC4291]".to_owned(),
                    allocation_date: "February 2006".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "::/128".parse().unwrap(),
                    name: "Unspecified Address".to_owned(),
                    rfc: "[RFC4291]".to_owned(),
                    allocation_date: "February 2006".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "64:ff9b::/96".parse().unwrap(),
                    name: "IPv4-IPv6 Translat.".to_owned(),
                    rfc: "[RFC6052]".to_owned(),
                    allocation_date: "October 2010".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: true,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "::ffff:0.0.0.0/96".parse().unwrap(),
                    name: "IPv4-mapped Address".to_owned(),
                    rfc: "[RFC4291]".to_owned(),
                    allocation_date: "February 2006".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
                Rfc6890Entry {
                    address_block: "100::/64".parse().unwrap(),
                    name: "Discard-Only Address Block".to_owned(),
                    rfc: "[RFC6666]".to_owned(),
                    allocation_date: "June 2012".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2001::/23".parse().unwrap(),
                    name: "IETF Protocol Assignments".to_owned(),
                    rfc: "[RFC2928]".to_owned(),
                    allocation_date: "September 2000".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2001::/32".parse().unwrap(),
                    name: "TEREDO".to_owned(),
                    rfc: "[RFC4380]".to_owned(),
                    allocation_date: "January 2006".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2001:2::/48".parse().unwrap(),
                    name: "Benchmarking".to_owned(),
                    rfc: "[RFC5180]".to_owned(),
                    allocation_date: "April 2008".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2001:db8::/32".parse().unwrap(),
                    name: "Documentation".to_owned(),
                    rfc: "[RFC3849]".to_owned(),
                    allocation_date: "July 2004".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2001:10::/28".parse().unwrap(),
                    name: "ORCHID".to_owned(),
                    rfc: "[RFC4843]".to_owned(),
                    allocation_date: "March 2007".to_owned(),
                    termination_date: "March 2014".to_owned(),
                    source: false,
                    destination: false,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "2002::/16".parse().unwrap(),
                    name: "6to4".to_owned(),
                    rfc: "[RFC3056]".to_owned(),
                    allocation_date: "February 2001".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "fc00::/7".parse().unwrap(),
                    name: "Unique-Local".to_owned(),
                    rfc: "[RFC4193]".to_owned(),
                    allocation_date: "October 2005".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: true,
                    global: false,
                    reserved_by_protocol: false,
                },
                Rfc6890Entry {
                    address_block: "fe80::/10".parse().unwrap(),
                    name: "Linked-Scoped Unicast".to_owned(),
                    rfc: "[RFC4291]".to_owned(),
                    allocation_date: "February 2006".to_owned(),
                    termination_date: "N/A".to_owned(),
                    source: true,
                    destination: true,
                    forwardable: false,
                    global: false,
                    reserved_by_protocol: true,
                },
            ],
        }
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
    use std::net::IpAddr;

    #[test]
    fn get_interface_ip_192() {
        let all: IpNet = "192.0.0.0/24".parse().unwrap();
        let specific: IpNet = "192.0.0.0/29".parse().unwrap();
        let rfc = Rfc6890::create();
        for ip_addr in all.hosts() {
            let is_forwardable = specific.contains(&ip_addr);
            assert_eq!(is_forwardable, rfc.is_forwardable(&ip_addr), "failure on ip {}", ip_addr)
        }
    }
}
