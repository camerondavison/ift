use crate::rfc::WithRfc6890;
use pest::{
    error::Error,
    iterators::Pair,
    Parser,
};
use pest_derive::*;
use pnet::datalink::{
    self,
    NetworkInterface,
};
use std::{
    cmp::Ordering,
    net::IpAddr,
    rc::Rc,
};

pub mod rfc;

#[derive(Parser)]
#[grammar = "ift.pest"]
struct IfTParser;

pub fn eval(s: &str) -> Vec<IpAddr> {
    match parse_ift_string(s) {
        Ok(parsed) => parsed
            .result
            .into_iter()
            .map(|ip2ni| ip2ni.ip_addr)
            .collect(),
        Err(err) => {
            eprintln!("{}", err);
            vec![]
        }
    }
}

#[derive(Debug)]
pub struct Ip2NetworkInterface {
    ip_addr: IpAddr,
    // 1 network interface can have multiple ips, but this way we can filter on both of them
    // all it takes is doing the cross product at the beginning
    interface: Option<Rc<NetworkInterface>>,
}

fn filter_by_flag(ip: &Ip2NetworkInterface, flag: &str) -> bool {
    match ip.interface.clone() {
        Some(int) => match flag {
            "up" => int.is_up(),
            "down" => !int.is_up(),
            _ => unreachable!("unknown flag [{}]", flag),
        },
        _ => false,
    }
}

fn filter_by_name(ip: &Ip2NetworkInterface, interface_name: &str) -> bool {
    match ip.interface.clone() {
        Some(int) => int.name == interface_name,
        _ => false,
    }
}

fn all_interfaces() -> Vec<Ip2NetworkInterface> {
    let interfaces = datalink::interfaces();
    let mut ret: Vec<Ip2NetworkInterface> = vec![];
    for interface in interfaces {
        let rc = Rc::new(interface);
        for ipn in (*rc.ips).iter() {
            ret.push(Ip2NetworkInterface {
                ip_addr: ipn.ip(),
                interface: Some(rc.clone()),
            })
        }
    }
    ret
}

fn sort_default_less(
    default_interface_name: String,
) -> impl FnMut(&Ip2NetworkInterface, &Ip2NetworkInterface) -> Ordering {
    move |a, b| {
        if let Some(ref ifa) = a.interface {
            if let Some(ref ifb) = b.interface {
                if ifa.name == default_interface_name {
                    return Ordering::Less;
                } else if ifb.name == default_interface_name {
                    return Ordering::Greater;
                }
            }
        }
        return Ordering::Equal;
    }
}

#[derive(Debug)]
struct IfTResult {
    result: Vec<Ip2NetworkInterface>,
}

fn parse_ift_string(template_str: &str) -> Result<IfTResult, Error<Rule>> {
    fn parse_producer(pair: Pair<Rule>) -> IfTResult {
        match pair.as_rule() {
            Rule::GetInterfaceIP => {
                let interface_name = pair.into_inner().next().unwrap().as_str();
                rule_filter_name(all_interfaces(), interface_name)
            }
            Rule::GetAllInterfaces => IfTResult {
                result: all_interfaces(),
            },
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn rule_filter_name(iter: Vec<Ip2NetworkInterface>, name: &str) -> IfTResult {
        IfTResult {
            result: iter
                .into_iter()
                .filter(|ip| filter_by_name(ip, name))
                .collect(),
        }
    }

    fn parse_filter(prev: IfTResult, pair: Pair<Rule>, rfc: &WithRfc6890) -> IfTResult {
        match pair.as_rule() {
            Rule::FilterIPv4 => IfTResult {
                result: prev
                    .result
                    .into_iter()
                    .filter(|ip2if| ip2if.ip_addr.is_ipv4())
                    .collect(),
            },
            Rule::FilterIPv6 => IfTResult {
                result: prev
                    .result
                    .into_iter()
                    .filter(|ip2if| ip2if.ip_addr.is_ipv6())
                    .collect(),
            },
            Rule::FilterName => {
                let name = pair.into_inner().next().unwrap().as_str();
                rule_filter_name(prev.result, name)
            }
            Rule::FilterFlags => {
                let flag = pair.into_inner().next().unwrap().as_str();
                IfTResult {
                    result: prev
                        .result
                        .into_iter()
                        .filter(|ip| filter_by_flag(ip, flag))
                        .collect(),
                }
            }
            Rule::FilterForwardable => IfTResult {
                result: prev
                    .result
                    .into_iter()
                    .filter(|ip| rfc.is_forwardable(&ip.ip_addr))
                    .collect(),
            },
            Rule::FilterGlobal => IfTResult {
                result: prev
                    .result
                    .into_iter()
                    .filter(|ip| rfc.is_global(&ip.ip_addr))
                    .collect(),
            },
            Rule::FilterFirst => IfTResult {
                result: prev.result.into_iter().next().into_iter().collect(),
            },
            Rule::FilterLast => IfTResult {
                result: prev.result.into_iter().last().into_iter().collect(),
            },
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn parse_sort(prev: IfTResult, pair: Pair<Rule>) -> IfTResult {
        let default_interface = read_default_interface_name();

        match pair.as_rule() {
            Rule::SortBy => {
                let attribute: &str = pair.into_inner().next().unwrap().as_str();
                let mut result = prev.result;
                result.sort_by(match attribute {
                    "default" => sort_default_less(default_interface),
                    _ => unimplemented!("nothing implemented for sort by [{}]", attribute),
                });
                IfTResult { result }
            }
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn parse_value(pair: Pair<Rule>, rfc: &WithRfc6890) -> IfTResult {
        match pair.as_rule() {
            Rule::expression => {
                let mut iter = pair.into_inner();
                let producer_pair = iter.next().unwrap().into_inner().next().unwrap();
                let mut base: IfTResult = parse_producer(producer_pair);
                for p in iter {
                    match p.as_rule() {
                        Rule::filter => {
                            base = parse_filter(base, p.into_inner().next().unwrap(), rfc)
                        }
                        Rule::sort => base = parse_sort(base, p.into_inner().next().unwrap()),
                        _ => unreachable!(
                            "only filters and sorts should follow. saw {:?}",
                            p.as_rule()
                        ),
                    }
                }
                base
            }
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    let template = IfTParser::parse(Rule::template, template_str)?
        .next()
        .unwrap();
    let rfc: WithRfc6890 = WithRfc6890::create();
    Ok(parse_value(template, &rfc))
}

fn read_default_interface_name() -> String {
    use std::process::Command;

    if cfg!(target_os = "linux") {
        parse_linux_ip_cmd(
            &String::from_utf8(
                Command::new("ip")
                    .arg("route")
                    .output()
                    .expect("failed to execute ip")
                    .stdout,
            )
            .unwrap(),
        )
    } else if cfg!(target_os = "macos") {
        parse_mac_ip_cmd(
            &String::from_utf8(
                Command::new("route")
                    .arg("-n")
                    .arg("get")
                    .arg("default")
                    .output()
                    .expect("failed to execute route")
                    .stdout,
            )
            .unwrap(),
        )
    } else {
        unimplemented!("target os not implemented")
    }
}

fn parse_mac_ip_cmd(output: &str) -> String {
    for line in output.split('\n') {
        let line: &str = line.trim();
        if line.starts_with("interface:") {
            return line["interface:".len()..].trim().to_owned();
        }
    }
    return "".to_owned();
}

fn parse_linux_ip_cmd(output: &str) -> String {
    for line in output.split('\n') {
        let line: &str = line.trim();
        if line.starts_with("default ") {
            return line.split(' ').last().unwrap().to_owned();
        }
    }
    return "".to_owned();
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use crate::{
        eval,
        parse_linux_ip_cmd,
        parse_mac_ip_cmd,
    };

    #[test]
    fn get_interface_ip() { assert!(eval("GetInterfaceIP \"eth30\"").is_empty()) }

    #[test]
    fn get_interface_ip_and_filter() {
        let eval_str = "GetInterfaceIP \"lo0\" | FilterIPv4";
        let expected: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(eval(eval_str).into_iter().next().unwrap(), expected)
    }

    #[test]
    fn get_private_ips() {
        let eval_str = "GetAllInterfaces | FilterForwardable | FilterFlags \"up\"";
        assert!(eval(eval_str).into_iter().next().is_some())
    }

    #[test]
    fn test_parse_mac() {
        let out = "\
           route to: default
destination: default
       mask: default
    gateway: 192.168.86.1
  interface: en0
      flags: <UP,GATEWAY,DONE,STATIC,PRCLONING>
 recvpipe  sendpipe  ssthresh  rtt,msec    rttvar  hopcount      mtu     expire
       0         0         0         0         0         0      1500         0";
        assert_eq!("en0", parse_mac_ip_cmd(out))
    }

    #[test]
    fn test_parse_linux() {
        let out = "\
        default via 172.17.0.1 dev eth0
        172.17.0.0/16 dev eth0 scope link  src 172.17.0.16";
        assert_eq!("eth0", parse_linux_ip_cmd(out))
    }
}