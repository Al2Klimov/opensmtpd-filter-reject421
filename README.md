## About

OpenSMTPd filter which wraps another OpenSMTPd filter and turns SMTP-550
reject into 999, so that OpenSMTPd itself replies with 421 (internal error).

This way, you don't accept rejected eMails, but don't tell that
the sender explicitly. **Use only with filters that 550-reject only spam!**
With this wrapper, you can play dumb and hope that spammers don't adapt
as they don't know you caught (and blocked) them.

## Build

Compile like any other Rust program: `cargo build -r`

Find the resulting binary directly under `target/release/`.

## Usage

`opensmtpd-filter-reject421 EXE [ARGS...]`

This filter will run the wrapped filter, `EXE ARGS...`, as subprocess,
forward stdio from/to it and manipulate its output as described above.

Integrate any wrapped filter into smtpd.conf(5).
