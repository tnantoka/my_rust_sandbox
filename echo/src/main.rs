use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echo")
        .version("0.1.0")
        .author("tnantoka <tnantoka@bornneet.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .allow_invalid_utf8(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    #[allow(unused_imports)]
    use predicates::prelude::*;

    #[test]
    fn it_runs() {
        runs(&["hello"], "hello\n");
    }

    #[test]
    fn it_runs_with_multiple_texts() {
        runs(&["hello", "world"], "hello world\n");
    }

    #[test]
    fn it_runs_with_n() {
        runs(&["hello", "-n"], "hello");
    }

    fn runs(args: &[&str], expected: &'static str) {
        Command::cargo_bin("echo")
            .unwrap()
            .args(args)
            .assert()
            .success()
            .stdout(expected);
    }

    #[test]
    fn it_dies_with_no_args() {
        let mut cmd = Command::cargo_bin("echo").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
    }
}
