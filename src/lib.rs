//! output first or last n lines, like a head and tail of linux command.
//!
//! ```text
//! Usage:
//!   aki-unbody [options]
//!
//! output first or last n lines, like a head and tail of linux command.
//!
//! Options:
//!   -h, --head <num>          output the first <num> lines.
//!   -t, --tail <num>          output the last <num> lines.
//!   -i, --inverse             output the body, except for head and tail.
//!
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//! ```
//!
//! # Examples
//!
//! The input data used in this example looks like this:
//!
//! ```text
//! cat file1.txt
//! ```
//!
//! result output:
//! ```text
//! LN:0001,text
//! LN:0002,text
//! LN:0003,text
//! LN:0004,text
//! LN:0005,text
//! LN:0006,text
//! ```
//!
//! ## Example 1: output head
//!
//! Outputs first 2 lines.
//!
//! command line:
//! ```text
//! cat file1.txt | aki-unbody --head 2
//! ```
//!
//! result output:
//! ```text
//! LN:0001,text
//! LN:0002,text
//! ```
//!
//! ## Example 2: output tail
//!
//! Outputs last 2 lines.
//!
//! command line:
//! ```text
//! cat file1.txt | aki-unbody --tail 2
//! ```
//!
//! result output:
//! ```text
//! LN:0005,text
//! LN:0006,text
//! ```
//!
//! ## Example 3: output head and tail
//!
//! Outputs first 2 lines and last 2 lines.
//!
//! command line:
//! ```text
//! cat file1.txt | aki-unbody --head 2 --tail 2
//! ```
//!
//! result output:
//! ```text
//! LN:0001,text
//! LN:0002,text
//! LN:0005,text
//! LN:0006,text
//! ```
//!
//! ## Example 4: output body, except for head and tail
//!
//! Outputs body, except for first 2 lines and last 2 lines.
//!
//! command line:
//! ```text
//! cat file1.txt | aki-unbody --head 2 --tail 2 --inverse
//! ```
//!
//! result output:
//! ```text
//! LN:0003,text
//! LN:0004,text
//! ```
//!
//! # Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute

#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute unbody
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "unbody"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: output head
///
/// Outputs first 2 lines.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_unbody::execute(&RunnelIoeBuilder::new().build(),
///     "unbody", &["--head", "2"]);
/// ```
///
/// ## Example 2: output tail
///
/// Outputs last 2 lines.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_unbody::execute(&RunnelIoeBuilder::new().build(),
///     "unbody", &["--tail", "2"]);
/// ```
///
/// ## Example 3: output head and tail
///
/// Outputs first 2 lines and last 2 lines.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_unbody::execute(&RunnelIoeBuilder::new().build(),
///     "unbody", &["--head", "2", "--tail", "2"]);
/// ```
///
/// ## Example 4: output body, except for head and tail
///
/// Outputs body, except for first 2 lines and last 2 lines.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_unbody::execute(&RunnelIoeBuilder::new().build(),
///     "unbody", &["--head", "2", "--tail", "2", "--inverse"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
