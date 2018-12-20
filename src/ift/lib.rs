use std::net::IpAddr;

use crate::ip_rfc::Rfc6890;
use pest::error::Error;
use pest::Parser;
use pest_derive::*;
use pnet::datalink::{self, NetworkInterface};
use std::rc::Rc;

mod ip_rfc;

#[derive(Parser)]
#[grammar = "ift/ift.pest"]
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

#[derive(Debug)]
struct IfTResult {
    result: Vec<Ip2NetworkInterface>,
}

fn parse_ift_string(template_str: &str) -> Result<IfTResult, Error<Rule>> {
    let template = IfTParser::parse(Rule::template, template_str)?
        .next()
        .unwrap();
    let rfc: Rfc6890 = Rfc6890::create();

    use pest::iterators::Pair;
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

    fn parse_filter(prev: IfTResult, pair: Pair<Rule>, rfc: &Rfc6890) -> IfTResult {
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
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn parse_value(pair: Pair<Rule>, rfc: &Rfc6890) -> IfTResult {
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
                        _ => unreachable!("only filters should follow. saw {:?}", p.as_rule()),
                    }
                }
                base
            }
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    Ok(parse_value(template, &rfc))
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use crate::eval;

    #[test]
    fn get_interface_ip() {
        assert!(eval("GetInterfaceIP \"eth30\"").is_empty())
    }

    #[test]
    fn get_interface_ip_and_filter() {
        let eval_str = "GetInterfaceIP \"lo0\" | FilterIPv4";
        let expected: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(eval(eval_str).into_iter().next().unwrap(), expected)
    }

    #[test]
    fn get_private_ips() {
        let eval_str = "GetAllInterfaces | FilterGlobal | FilterFlags \"up\"";
        let expected: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(eval(eval_str).into_iter().next().unwrap(), expected)
    }
}
