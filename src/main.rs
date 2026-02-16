//! Main CLI entry point for Log Security Analyzer.

use log::info;

mod rules;
mod scanner;
mod severity;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <log_file> <rules_file>", args[0]);
        std::process::exit(1);
    }

    let log_file = &args[1];
    let rules_file = &args[2];

    let rules = rules::load_rules(rules_file)?;
    info!("Loaded {} rules", rules.len());

    scanner::scan_log(log_file, &rules)?;

    Ok(())
}
