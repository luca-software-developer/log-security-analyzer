<img width="144" height="144" alt="log-security-analyzer" src="assets/logo.png" />

# Log Security Analyzer

![Version: 1.0.0](https://img.shields.io/badge/version-1.0.0-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-blue)

A Rust CLI tool to scan log files and detect exposed secrets (tokens, API keys,
credentials) using configurable regex rules in TOML format.

## Features

- Line-by-line scanning of arbitrary log files
- Detection rules defined in TOML files, easily extensible
- Advanced regex support (lookahead/lookbehind) via `fancy-regex`
- Severity levels: `critical`, `high`, `medium`, `low`
- Formatted table output with color-coded severity
- Results sorted by descending severity

## Included Rules

The default ruleset (`rulesets/default.toml`) detects:

| Secret                       | Severity |
|------------------------------|----------|
| GitHub Personal Access Token | critical |
| AWS Secret Access Key        | critical |
| Private Key (PEM)            | critical |
| Slack Bot Token              | high     |
| AWS Access Key ID            | high     |
| PostgreSQL Connection String | high     |
| MySQL Connection String      | high     |
| JWT Token                    | high     |

## Requirements

- Rust 1.70+

## Build

```bash
cargo build --release
```

The binary is generated at `target/release/log-security-analyzer`.

## Usage

```bash
log-security-analyzer <log_file> <rules_file>
```

Example with the files included in the repository:

```bash
cargo run -- logs/app.log rulesets/default.toml
```

### Debug Logging

The tool uses `env_logger`. To enable internal logs:

```bash
RUST_LOG=info cargo run -- logs/app.log rulesets/default.toml
```

## Creating Custom Rules

Create a `.toml` file with the following structure:

```toml
[[rules]]
id = "my-rule"
description = "Description of the secret"
regex = '''pattern_regex'''
tags = ["tag1", "tag2"]
severity = "high"
```

Valid values for `severity`: `critical`, `high`, `medium`, `low`.

## Project Structure

```
src/
  main.rs      # CLI entry point
  lib.rs       # Public library interface
  rules.rs     # Rule parsing from TOML
  scanner.rs   # Scanning logic and table output
  severity.rs  # Severity level enum
rulesets/
  default.toml # Default ruleset
logs/
  app.log      # Sample log file
```

## Testing

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for full details.

Copyright (c) 2026 Luca Dello Russo
