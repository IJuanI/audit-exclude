extern crate docopt;
#[macro_use]
extern crate serde_derive;

extern crate audit_exclude;

use audit_exclude::{run, version};
use docopt::Docopt;

const USAGE: &str = "
audit-exclude excludes the output of \"npm audit --json\"

Usage:
  audit-exclude [--json] [--audit=<->] [--nsp-config=<.nsprc>]
  audit-exclude (-h | --help | --version)

Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --json                          Output subset of JSON for the unexcluded vulnerabilities as an array.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default exclude config [default: .nsprc].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_audit: String,
    flag_nsp_config: String,
    // flag_version: bool,
    flag_json: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());

    ::std::process::exit(run(&args.flag_audit, &args.flag_nsp_config, args.flag_json));
}
