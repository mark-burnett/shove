use docopt;

static USAGE: &'static str = "
Usage: shove [options]
       shove --help

Options:
  -h, --help                Show this message.
  -l, --log-level LEVEL     Log level [default: INFO].
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_log_level: String,
}

pub fn parse_args() -> Args {
    docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}
