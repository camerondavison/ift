use std::net::IpAddr;

use pest::error::Error;
use pest::Parser;
use pest_derive::*;
use pnet::datalink::{self, NetworkInterface};

#[derive(Parser)]
#[grammar = "ift/ift.pest"]
struct IfTParser;

pub fn eval(s: &str) -> Vec<IpAddr> {
    match parse_ift_string(s) {
        Ok(parsed) => parsed.result,
        Err(err) => {
            eprintln!("{}", err);
            vec![]
        }
    }
}

fn read_iface(interface_name: &str) -> Vec<IpAddr> {
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    let interfaces = datalink::interfaces();
    if let Some(interface) = interfaces.into_iter().find(interface_names_match) {
        return interface.ips.iter().map(|ip_net| ip_net.ip()).collect();
    } else {
        return vec![];
    }
}

#[derive(Debug)]
struct IfTResult {
    result: Vec<IpAddr>,
}

fn parse_ift_string(template_str: &str) -> Result<IfTResult, Error<Rule>> {
    let template = IfTParser::parse(Rule::template, template_str)?
        .next()
        .unwrap();

    use pest::iterators::Pair;
    fn parse_producer(pair: Pair<Rule>) -> IfTResult {
        match pair.as_rule() {
            Rule::GetInterfaceIP => {
                let interface_name = pair.into_inner().next().unwrap().as_str();
                IfTResult {
                    result: read_iface(interface_name),
                }
            }
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn parse_filter(prev: IfTResult, pair: &Pair<Rule>) -> IfTResult {
        match pair.as_rule() {
            Rule::FilterIPv4 => IfTResult {
                result: prev.result.into_iter().filter(|ip| ip.is_ipv4()).collect(),
            },
            Rule::FilterIPv6 => IfTResult {
                result: prev.result.into_iter().filter(|ip| ip.is_ipv6()).collect(),
            },
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    fn parse_value(pair: Pair<Rule>) -> IfTResult {
        match pair.as_rule() {
            Rule::expression => {
                let mut iter = pair.into_inner();
                let producer_pair = iter.next().unwrap().into_inner().next().unwrap();
                let mut base: IfTResult = parse_producer(producer_pair);
                for p in iter {
                    match p.as_rule() {
                        Rule::filter => base = parse_filter(base, &p.into_inner().next().unwrap()),
                        _ => unreachable!("only filters should follow. saw {:?}", p.as_rule()),
                    }
                }
                base
            }
            _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
        }
    }

    Ok(parse_value(template))
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use crate::eval;

    #[test]
    fn get_interface_ip() {
        let empty: Vec<IpAddr> = vec![];
        assert_eq!(eval("GetInterfaceIP \"eth30\""), empty)
    }

    #[test]
    fn get_interface_ip_and_filter() {
        let expected: Vec<IpAddr> = vec!["127.0.0.1".parse().unwrap()];
        assert_eq!(eval("GetInterfaceIP \"lo0\" | FilterIPv4"), expected)
    }
}
