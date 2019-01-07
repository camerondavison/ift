#![recursion_limit = "1024"]

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
mod routes;
use crate::{
    rfc::WithRfc6890,
    routes::read_default_interface_name,
};

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{
    foreign_links {
    Utf8(::std::string::FromUtf8Error);
    Io(::std::io::Error);


    }
    }
}

/// # Evaluate a interface template
///
/// Given an expression, return a list of IpAddr's that match.
///
/// Starting with one producer, use the pipe | character to filter and sort
/// what IpAddr's will be returned. All <arguments> are quoted with ". One interface
/// can produce multiple IpAddr's. In mac lo0 produces some IPv4 and some IPv6 addresses.
///
/// ### producers
/// GetAllInterfaces
///   - Get all the interfaces available
///
/// GetInterface <name>
///   - Short for `GetAllInterfaces | FilterName "name"`
/// ```
/// use ift::eval;
/// assert_eq!(eval("GetInterface \"en0\""), eval("GetAllInterfaces | FilterName \"en0\""));
/// ```
///
/// ### filters
/// Filter the IpAddr's that were produced. If an interface produces multiple IpAddrs then the
/// information about that interface is copied to the other IpAddrs. This means that filters
/// can be on either the interface attributes or the ip attributes along the way.
///
/// FilterIPv4
///   - Filter to only ipv4 ips
///
/// FilterIPv6
///   - Filter to only ipv6 ips
///
/// FilterFlags <flag>
///   - Filter by flags "up"/"down"
///
/// FilterName <interface name>
///   - Filter by a specified interface name
///
/// FilterForwardable
///   - Filter on whether or not it is forwaradable according to [RFC6890](https://tools.ietf.org/rfc/rfc6890.txt)
///
/// FilterGlobal
///   - Filter on whether or not it is global according to [RFC6890](https://tools.ietf.org/rfc/rfc6890.txt)
///
/// FilterFirst/FilterLast
///   - Only return either the first IpAddr or the last IpAddr
///
/// ### sorts
/// SortBy <attribute>
///   - Sort by attribute "default", looks up the default interface and sorts it to the front
///
/// ```
/// use ift::eval;
/// assert_eq!(false, eval("GetAllInterfaces").is_empty());
/// assert_eq!(true, eval("GetAllInterfaces | FilterIPv4 | FilterIPv6").is_empty());
/// ```
pub fn eval(s: &str) -> Vec<IpAddr> {
    match parse_ift_string(s) {
        Ok(parsed) => parsed.result.into_iter().map(|ip2ni| ip2ni.ip_addr).collect(),
        Err(err) => {
            eprintln!("{}", err);
            vec![]
        }
    }
}

#[derive(Parser)]
#[grammar = "ift.pest"]
struct IfTParser;

#[derive(Debug)]
struct Ip2NetworkInterface {
    ip_addr: IpAddr,
    // 1 network interface can have multiple ips, but this way we can filter on both of them
    // all it takes is doing the cross product at the beginning
    interface: Option<Rc<NetworkInterface>>,
}

#[derive(Debug)]
struct IfTResult {
    result: Vec<Ip2NetworkInterface>,
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
        Ordering::Equal
    }
}

fn parse_ift_string(template_str: &str) -> Result<IfTResult, Error<Rule>> {
    let template = IfTParser::parse(Rule::template, template_str)?.next().unwrap();
    let rfc: WithRfc6890 = WithRfc6890::create();
    Ok(parse_value(template, &rfc))
}

fn parse_producer(pair: Pair<Rule>) -> IfTResult {
    match pair.as_rule() {
        Rule::GetInterface => {
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
        result: iter.into_iter().filter(|ip| filter_by_name(ip, name)).collect(),
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
                result: prev.result.into_iter().filter(|ip| filter_by_flag(ip, flag)).collect(),
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
    let default_interface = read_default_interface_name().expect("unable to find default interface");

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
                    Rule::filter => base = parse_filter(base, p.into_inner().next().unwrap(), rfc),
                    Rule::sort => base = parse_sort(base, p.into_inner().next().unwrap()),
                    _ => unreachable!("only filters and sorts should follow. saw {:?}", p.as_rule()),
                }
            }
            base
        }
        _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
    }
}
