use pnet::datalink::{self, NetworkInterface};
use std::net::IpAddr;

pub fn eval(s: String) -> Vec<IpAddr> {
    return read_iface(s);
}

fn read_iface(interface_name: String) -> Vec<IpAddr> {
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    return interface.ips.iter().map(|ip_net| ip_net.ip()).collect();
}
