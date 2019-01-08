#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn all() {
        let cmds = if cfg!(target_os = "macos") {
            vec![
                (["eval", "GetInterface \"lo0\" | FilterIPv4"], "[127.0.0.1]\n"),
                (["eval", "GetInterface \"lo0\" | FilterIPv6 | FilterFirst"], "[::1]\n"),
            ]
        } else {
            vec![(["eval", "GetInterface \"lo\" | FilterIPv4"], "[127.0.0.1]\n")]
        };
        for (cmd, stdout) in cmds {
            Command::cargo_bin("ift").unwrap().args(&cmd).assert().stdout(stdout);
        }
    }
}
