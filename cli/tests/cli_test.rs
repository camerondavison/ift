#[cfg(test)]
mod tests {
    use std::process::Command;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn all() {
        let cmds = vec![
            (["eval", "GetInterfaceIP \"lo0\" | FilterIPv4"], "[127.0.0.1]\n"),
            (["eval", "GetInterfaceIP \"lo0\" | FilterIPv6 | FilterFirst"], "[::1]\n"),
        ];
        for (cmd, stdout) in cmds {
            Command::main_binary().unwrap()
                .args(&cmd)
                .assert()
                .stdout(stdout);
        }
    }
}