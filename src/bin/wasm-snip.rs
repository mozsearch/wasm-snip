use std::fs;
use std::io::{self, Write};
use std::process;

use anyhow::Context;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        for c in e.chain().skip(1) {
            eprintln!("  caused by {}", c);
        }
        eprintln!("{}", e.backtrace());
        process::exit(1)
    }
}

fn try_main() -> Result<(), anyhow::Error> {
    let matches = parse_args();

    let mut opts = wasm_snip::Options::default();

    opts.functions = matches
        .get_many::<String>("function")
        .map(|fs| fs.map(|f| f.to_string()).collect())
        .unwrap_or(vec![]);

    opts.patterns = matches
        .get_many::<String>("pattern")
        .map(|ps| ps.map(|p| p.to_string()).collect())
        .unwrap_or(vec![]);

    if matches.get_flag("snip_rust_fmt_code") {
        opts.snip_rust_fmt_code = true;
    }

    if matches.get_flag("snip_rust_panicking_code") {
        opts.snip_rust_panicking_code = true;
    }

    if matches.get_flag("skip_producers_section") {
        opts.skip_producers_section = true;
    }

    let config = walrus_config_from_options(&opts);
    let path = matches
        .get_one::<String>("input")
        .map(|s| s.as_str())
        .unwrap();
    let buf = fs::read(&path).with_context(|| format!("failed to read file {}", path))?;
    let mut module = config.parse(&buf)?;

    wasm_snip::snip(&mut module, opts).context("failed to snip functions from wasm module")?;

    if let Some(output) = matches.get_one::<String>("output").map(|s| s.as_str()) {
        module
            .emit_wasm_file(output)
            .with_context(|| format!("failed to emit snipped wasm to {}", output))?;
    } else {
        let wasm = module.emit_wasm();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout
            .write_all(&wasm)
            .context("failed to write wasm to stdout")?;
    }

    Ok(())
}

fn walrus_config_from_options(options: &wasm_snip::Options) -> walrus::ModuleConfig {
    let mut config = walrus::ModuleConfig::new();
    config.generate_producers_section(!options.skip_producers_section);
    config
}

fn parse_args() -> clap::ArgMatches {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            "
`wasm-snip` replaces a WebAssembly function's body with an `unreachable`.

Maybe you know that some function will never be called at runtime, but the
compiler can't prove that at compile time? Snip it! Then run `wasm-gc`[0] again
and all the functions it transitively called (which could also never be called
at runtime) will get removed too.

Very helpful when shrinking the size of WebAssembly binaries!

[0]: https://github.com/alexcrichton/wasm-gc
",
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .action(clap::ArgAction::Set)
                .help("The path to write the output wasm file to. Defaults to stdout."),
        )
        .arg(
            clap::Arg::new("input")
                .required(true)
                .action(clap::ArgAction::Set)
                .help("The input wasm file containing the function(s) to snip."),
        )
        .arg(
            clap::Arg::new("function")
                .action(clap::ArgAction::Append)
                .help(
                    "The specific function(s) to snip. These must match \
             exactly. Use the -p flag for fuzzy matching.",
                ),
        )
        .arg(
            clap::Arg::new("pattern")
                .required(false)
                .short('p')
                .long("pattern")
                .action(clap::ArgAction::Append)
                .help("Snip any function that matches the given regular expression."),
        )
        .arg(
            clap::Arg::new("snip_rust_fmt_code")
                .required(false)
                .long("snip-rust-fmt-code")
                .action(clap::ArgAction::SetTrue)
                .help("Snip Rust's `std::fmt` and `core::fmt` code."),
        )
        .arg(
            clap::Arg::new("snip_rust_panicking_code")
                .required(false)
                .long("snip-rust-panicking-code")
                .action(clap::ArgAction::SetTrue)
                .help("Snip Rust's `std::panicking` and `core::panicking` code."),
        )
        .arg(
            clap::Arg::new("skip_producers_section")
                .required(false)
                .long("skip-producers-section")
                .action(clap::ArgAction::SetTrue)
                .help("Do not emit the 'producers' custom section."),
        )
        .get_matches()
}
