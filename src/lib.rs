#![deny(missing_docs)]
//! # IFT (interface templates)
//!
//! Template strings to extract the correct interface and IpAddr to bind your to.
//! Heavily inspired by https://github.com/hashicorp/go-sockaddr
//!
//! ## What is it?
//! `eval`([docs](https://camerondavison.github.io/ift/ift/fn.eval.html#evaluate-a-interface-template))
//! takes an interface template string. The template is a string that starts with a
//! [producer](https://camerondavison.github.io/ift/ift/fn.eval.html#producers)
//! and is followed by [filters](https://camerondavison.github.io/ift/ift/fn.eval.html#filters)
//! and [sorts](https://camerondavison.github.io/ift/ift/fn.eval.html#sorts)
//! each of which is pipe `|` delimited. `eval` returns a vector of [IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html) objects
//! that can then be used as bindings
//!
//! ## Usage
//!
//! ### general
//! ```
//! use ift::eval;
//! print!("{:?}", eval(r#"GetInterface "en0""#).unwrap());
//! ```
//!
//! ### actix
//! ```
//! use actix_web::{
//!    server,
//!    App,
//! };
//! let mut s = server::new(|| { App::new() });
//! for ip in ift::eval("GetPrivateInterfaces").unwrap().into_iter() {
//!   s = s.bind((ip, 8080)).unwrap();
//! }
//! ```
//!
//! ### Example Templates
//! - get private interfaces
//!   `GetAllInterfaces | FilterFlags "up" | FilterForwardable | SortBy "default"`
//! - get private interfaces short
//!   `GetPrivateInterfaces`
//! - get specific interface by name
//!   `GetInterface "en0"`
//! - get only interfaces with ipv6 addresses
//!   `GetAllInterfaces | FilterIPv6`
//!
//! ## Examples
//! There are examples in the [examples](https://github.com/camerondavison/ift/tree/master/examples)
//! folder.
//! * [actix](https://github.com/camerondavison/ift/blob/master/examples/actix.rs) - bind multiple private interfaces
//!
use failure::{
    Error,
    Fail,
};
use pest::{
    iterators::Pair,
    Parser,
};
use pnet::datalink::{
    self,
    NetworkInterface,
};
use std::{
    cmp::Ordering,
    net::IpAddr,
    rc::Rc,
    str::FromStr,
};

pub mod rfc;
mod routes;
use crate::{
    rfc::WithRfc6890,
    routes::read_default_interface_name,
};
mod grammar;
use crate::grammar::{
    IfTParser,
    Rule,
};

/// Some errors that can come from the evaluation of the template
#[derive(Debug, Fail)]
pub enum IfTError {
    /// Error parsing string to utf8
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] ::std::string::FromUtf8Error),
    /// IO error reading template
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] ::std::io::Error),
    /// Pest parse error
    #[fail(display = "{}", _0)]
    Pest(::pest::error::Error<Rule>),
    /// Error parsing a flag
    #[fail(display = "unable to parse flag {}", _0)]
    IfTFlagError(String),
    /// Error parsing an argument
    #[fail(display = "unable to use argument {}", _0)]
    IfTArgumentError(String),
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
/// #### GetAllInterfaces
/// Get all the interfaces available
///
/// #### GetPrivateInterfaces
/// Get sorted list of interfaces available that are forwardable, and up. Sorted by default first.
///
/// Short for `GetAllInterfaces | FilterFlags "up" | SortBy "default"`
/// ```
/// use ift::eval;
/// assert_eq!(eval("GetPrivateInterfaces").unwrap(), eval(r#"GetAllInterfaces | FilterFlags "up" | FilterForwardable | SortBy "default""#).unwrap());
/// ```
///
/// #### GetInterface <name>
/// Short for `GetAllInterfaces | FilterName "name"`
/// ```
/// use ift::eval;
/// assert_eq!(eval("GetInterface \"en0\"").unwrap(), eval("GetAllInterfaces | FilterName \"en0\"").unwrap());
/// ```
///
/// ### filters
/// Filter the IpAddr's that were produced. If an interface produces multiple IpAddrs then the
/// information about that interface is copied to the other IpAddrs. This means that filters
/// can be on either the interface attributes or the ip attributes along the way.
///
/// #### FilterIPv4
///   Filter to only ipv4 ips
///
/// #### FilterIPv6
/// Filter to only ipv6 ips
///
/// #### FilterFlags <flag>
/// Filter by flags "up"/"down"
///
/// #### FilterName <interface name>
/// Filter by a specified interface name
///
/// #### FilterForwardable
/// Filter on whether or not it is forwaradable according to [RFC6890](https://tools.ietf.org/rfc/rfc6890.txt)
///
/// #### FilterGlobal
/// Filter on whether or not it is global according to [RFC6890](https://tools.ietf.org/rfc/rfc6890.txt)
///
/// #### FilterFirst/FilterLast
/// Only return either the first IpAddr or the last IpAddr
///
/// ### sorts
/// #### SortBy <attribute>
/// Sort by attribute "default", looks up the default interface and sorts it to the front
///
/// ```
/// use ift::evals;
/// assert_eq!(true, evals("GetAllInterfaces").is_some());
/// assert_eq!(false, evals("GetAllInterfaces | FilterIPv4 | FilterIPv6").is_some());
/// ```
pub fn eval(s: &str) -> Result<Vec<IpAddr>, Error> {
    let parsed = parse_ift_string(s)?;
    Ok(parsed.result.into_iter().map(|ip2ni| ip2ni.ip_addr).collect())
}

/// Just like `eval`.
/// Returns the first IpAddr as an option. None if empty vector.
pub fn evals(s: &str) -> Option<IpAddr> { eval(s).unwrap().into_iter().next() }

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

fn parse_ift_string(template_str: &str) -> Result<IfTResult, Error> {
    let template = IfTParser::parse(Rule::template, template_str)?.next().unwrap();
    let rfc: WithRfc6890 = WithRfc6890::create();
    Ok(parse_expression(template, &rfc)?)
}

enum IfTFlag {
    UP,
    DOWN,
}
impl FromStr for IfTFlag {
    type Err = IfTError;

    fn from_str(flag: &str) -> ::std::result::Result<Self, Self::Err> {
        match flag {
            "up" => Ok(IfTFlag::UP),
            "down" => Ok(IfTFlag::DOWN),
            _ => Err(IfTError::IfTFlagError(flag.to_owned())),
        }
    }
}

fn filter_by_flag(ip: &Ip2NetworkInterface, flag: &IfTFlag) -> bool {
    match ip.interface.clone() {
        Some(int) => match flag {
            IfTFlag::UP => int.is_up(),
            IfTFlag::DOWN => !int.is_up(),
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

fn rule_filter_name(iter: Vec<Ip2NetworkInterface>, name: &str) -> IfTResult {
    IfTResult {
        result: iter.into_iter().filter(|ip| filter_by_name(ip, name)).collect(),
    }
}

fn parse_expression(pair: Pair<'_, Rule>, rfc: &WithRfc6890) -> Result<IfTResult, Error> {
    match pair.as_rule() {
        Rule::expression => {
            let mut iter = pair.into_inner();
            let producer_pair = iter.next().unwrap().into_inner().next().unwrap();
            let mut base: IfTResult = parse_producer(producer_pair)?;

            for p in iter {
                match p.as_rule() {
                    Rule::filter => base = parse_filter(base, p.into_inner().next().unwrap(), rfc)?,
                    Rule::sort => base = parse_sort(base, p.into_inner().next().unwrap())?,
                    _ => unreachable!("only filters and sorts should follow. saw {:?}", p.as_rule()),
                }
            }
            Ok(base)
        }
        _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
    }
}

fn parse_producer(pair: Pair<'_, Rule>) -> Result<IfTResult, Error> {
    let rfc = WithRfc6890::create();

    match pair.as_rule() {
        Rule::GetInterface => {
            let interface_name = pair.into_inner().next().unwrap().as_str();
            Ok(rule_filter_name(all_interfaces(), interface_name))
        }
        Rule::GetAllInterfaces => Ok(IfTResult {
            result: all_interfaces(),
        }),
        Rule::GetPrivateInterfaces => rule_sort_by_attribute(
            IfTResult {
                result: all_interfaces()
                    .into_iter()
                    .filter(|ip| filter_by_flag(&ip, &IfTFlag::UP))
                    .filter(|ip| rfc.is_forwardable(&ip.ip_addr))
                    .collect(),
            },
            "default",
        ),
        _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
    }
}

fn parse_filter(prev: IfTResult, pair: Pair<'_, Rule>, rfc: &WithRfc6890) -> Result<IfTResult, Error> {
    Ok(match pair.as_rule() {
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
            let flag: IfTFlag = flag.parse()?;
            IfTResult {
                result: prev.result.into_iter().filter(|ip| filter_by_flag(ip, &flag)).collect(),
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
    })
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

fn parse_sort(prev: IfTResult, pair: Pair<'_, Rule>) -> Result<IfTResult, Error> {
    match pair.as_rule() {
        Rule::SortBy => {
            let attribute: &str = pair.into_inner().next().unwrap().as_str();
            rule_sort_by_attribute(prev, attribute)
        }
        _ => unreachable!("unable to parse rule {:?}", pair.as_rule()),
    }
}

fn rule_sort_by_attribute(prev: IfTResult, attribute: &str) -> Result<IfTResult, Error> {
    let default_interface = read_default_interface_name()?;
    let sorter = match attribute {
        "default" => Ok(sort_default_less(default_interface)),
        _ => Err(IfTError::IfTArgumentError(attribute.to_owned())),
    }?;
    let mut result = prev.result;
    result.sort_by(sorter);
    Ok(IfTResult { result })
}
