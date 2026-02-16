# log-security-analyzer

Tool CLI in Rust per scansionare file di log e individuare segreti esposti (
token, chiavi API, credenziali) tramite regole regex configurabili in formato
TOML.

## Funzionalità

- Scansione riga per riga di file di log arbitrari
- Regole di rilevamento definite in file TOML, facilmente estendibili
- Supporto a regex avanzate (lookahead/lookbehind) tramite `fancy-regex`
- Livelli di severità: `critical`, `high`, `medium`, `low`
- Output formattato in tabella con colori per severità
- Risultati ordinati per severità decrescente

## Regole incluse

Il ruleset di default (`rulesets/default.toml`) rileva:

| Segreto                      | Severità |
|------------------------------|----------|
| GitHub Personal Access Token | critical |
| AWS Secret Access Key        | critical |
| Private Key (PEM)            | critical |
| Slack Bot Token              | high     |
| AWS Access Key ID            | high     |
| PostgreSQL Connection String | high     |
| MySQL Connection String      | high     |
| JWT Token                    | high     |

## Requisiti

- Rust 1.70+

## Build

```bash
cargo build --release
```

Il binario viene generato in `target/release/log-security-analyzer`.

## Utilizzo

```bash
log-security-analyzer <log_file> <rules_file>
```

Esempio con i file inclusi nel repository:

```bash
cargo run -- logs/app.log rulesets/default.toml
```

### Logging di debug

Il tool usa `env_logger`. Per abilitare i log interni:

```bash
RUST_LOG=info cargo run -- logs/app.log rulesets/default.toml
```

## Creare regole personalizzate

Creare un file `.toml` con la seguente struttura:

```toml
[[rules]]
id = "my-rule"
description = "Descrizione del segreto"
regex = '''pattern_regex'''
tags = ["tag1", "tag2"]
severity = "high"
```

Valori validi per `severity`: `critical`, `high`, `medium`, `low`.

## Struttura del progetto

```
src/
  main.rs      # Entry point CLI
  lib.rs       # Interfaccia pubblica della libreria
  rules.rs     # Parsing delle regole da TOML
  scanner.rs   # Logica di scansione e output tabellare
  severity.rs  # Enum dei livelli di severità
rulesets/
  default.toml # Ruleset predefinito
logs/
  app.log      # File di log di esempio
```

## Test

```bash
cargo test
```

## Licenza

Questo progetto è distribuito sotto licenza MIT. Consulta il file [LICENSE](LICENSE) per i dettagli completi.

Copyright (c) 2026 Luca Dello Russo
