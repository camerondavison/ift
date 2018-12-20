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

struct Rfc6890Entry {
    address_block: IpNet,
    name: String,
    rfc: String,
    allocation_date: String,
    termination_date: String,
    source: bool,
    destination: bool,
    forwardable: bool,
    global: bool,
    reserved_by_protocol: bool,
}

pub struct Rfc6890 {
    entries: Vec<Rfc6890Entry>,
}

impl Rfc6890 {
    pub fn create() -> Rfc6890 {
        Rfc6890 {
            entries: vec![Rfc6890Entry {
                address_block: "10.0.0.0/8".parse().unwrap(),
                name: "Private-Use".to_owned(),
                rfc: "RFC1918".to_owned(),
                allocation_date: "February 1996".to_owned(),
                termination_date: "N/A".to_owned(),
                source: true,
                destination: true,
                forwardable: true,
                global: false,
                reserved_by_protocol: false,
            }],
        }
    }

    pub fn is_forwardable(&self, ip: &IpAddr) -> bool {
        for entry in &self.entries {
            if entry.address_block.contains(ip) {
                return entry.forwardable;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::ip_rfc::Rfc6890;
    use ipnet::IpNet;
    use std::net::IpAddr;

    #[test]
    fn get_interface_ip() {
        let net: IpNet = "192.0.0.0/29".parse().unwrap();
        let rfc = Rfc6890::create();
        for ip_addr in net.hosts() {
            assert!(rfc.is_forwardable(&ip_addr))
        }
    }
}
