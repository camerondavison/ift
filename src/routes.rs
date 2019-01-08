use failure::Error;
use std::process::Command;

pub fn read_default_interface_name() -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        Ok(parse_linux_ip_cmd(&String::from_utf8(
            Command::new("ip").arg("route").output()?.stdout,
        )?))
    } else if cfg!(target_os = "macos") {
        Ok(parse_mac_ip_cmd(&String::from_utf8(
            Command::new("route")
                .arg("-n")
                .arg("get")
                .arg("default")
                .output()?
                .stdout,
        )?))
    } else {
        unimplemented!("unimplemented os")
    }
}

fn parse_mac_ip_cmd(output: &str) -> String {
    for line in output.split('\n') {
        let line: &str = line.trim();
        if line.starts_with("interface:") {
            return line["interface:".len()..].trim().to_owned();
        }
    }
    "".to_owned()
}

fn parse_linux_ip_cmd(output: &str) -> String {
    for line in output.split('\n') {
        let line: &str = line.trim();
        if line.starts_with("default ") {
            return line.split(' ').last().unwrap().to_owned();
        }
    }
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use crate::routes::{
        parse_linux_ip_cmd,
        parse_mac_ip_cmd,
    };

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
