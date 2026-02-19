use std::borrow::Cow;
use std::env::args_os;
use std::io::{BufRead, BufReader, Result as IoResult, Write, stdout};
use std::process::{Command, Stdio, exit};

fn main() -> IoResult<()> {
    let mut args = args_os();
    let program = args.next();

    let exe = match args.next() {
        None => {
            eprintln!(
                "Usage: {} EXE [ARGS...]",
                match program {
                    None => Cow::from("opensmtpd-filter-reject421"),
                    Some(ref p) => p.to_string_lossy(),
                }
            );

            exit(1);
        }
        Some(v) => v,
    };

    let mut cmd = Command::new(exe.clone());

    cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    eprintln!("Running {}", exe.to_string_lossy());

    let mut child = match cmd.spawn() {
        Err(err) => {
            eprintln!("exec(3): {}", err);
            exit(1);
        }
        Ok(v) => v,
    };

    let ret = run(BufReader::new(child.stdout.take().unwrap()));

    let _ = child.wait();

    ret
}

fn run(mut input: impl BufRead) -> IoResult<()> {
    let mut std_out = stdout().lock();
    let mut line = Vec::<u8>::new();

    loop {
        line.clear();
        input.read_until(b'\n', &mut line)?;

        if line.is_empty() {
            return Ok(());
        }

        let mut f = line.split(|&sep| sep == b'|');

        match (f.next(), f.next(), f.next(), f.next(), f.next()) {
            (Some(b"filter-result"), Some(session), Some(token), Some(b"reject"), Some(reason)) => {
                if reason.starts_with(b"550 ") {
                    std_out.write_all(b"filter-result|")?;
                    std_out.write_all(session)?;
                    std_out.write_all(b"|")?;
                    std_out.write_all(token)?;
                    writeln!(std_out, "|reject|999 Error")?;
                } else {
                    std_out.write_all(line.as_slice())?;
                }
            }
            _ => {
                std_out.write_all(line.as_slice())?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn run_empty_input_returns_ok() {
        run(Cursor::new(b"" as &[u8])).unwrap();
    }

    #[test]
    fn run_passthrough_line_returns_ok() {
        run(Cursor::new(b"some line\n" as &[u8])).unwrap();
    }

    #[test]
    fn run_550_line_returns_ok() {
        run(Cursor::new(
            b"filter-result|abc|tok|reject|550 reason\n" as &[u8],
        ))
        .unwrap();
    }

    #[test]
    fn run_421_line_returns_ok() {
        run(Cursor::new(
            b"filter-result|abc|tok|reject|421 reason\n" as &[u8],
        ))
        .unwrap();
    }
}
