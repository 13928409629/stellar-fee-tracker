## CLI v2

DevKit ships with a brand-new v2 command-line interface, with a unified binary entry point `devkit`.
All commands below handle Stellar XLM/Stroops asset conversion, configuration validation, data diagnostics, environment management and related utilities.

### Global Shared Flags (available for all subcommands)
```bash
devkit -v/--verbose          # Enable detailed debug logs
devkit -q/--quiet             # Quiet mode, only print errors
devkit --log-level [debug|info|warn|error]  # Set log verbosity level
devkit -c/--config <PATH>    # Load custom configuration file
```bash

1. validate
Validate wallet / transaction / config files; detect XLM precision errors, out-of-bound values, invalid decimal digits and malformed structures.
```bash
# Basic: validate a single wallet JSON file
devkit validate ./wallets/user_wallet.json

# Strict mode: full validation with detailed error breakdown
devkit validate --strict --verbose ./transactions/batch.toml

# Batch validate all files in a directory
devkit validate --dir ./wallets/ --ext json,toml

# Only validate XLM & stroops balance fields, skip irrelevant metadata
devkit validate --filter xlm,stroops ./ledger_records.json
```

2. repair
Auto-fix invalid data flagged by validate: normalize XLM to 6 decimal places, resolve out-of-range stroops values and fix unit mismatches.
```bash
# In-place repair (overwrite original file)
devkit repair ./wallets/user_wallet.json

# Generate fixed output file without modifying source
devkit repair --output ./wallets/user_wallet_fixed.json ./wallets/user_wallet.json

# Batch repair all files in directory, append .fixed suffix to outputs
devkit repair --dir ./transactions/ --suffix .fixed --ext json
```
3. compare
Compare two config / transaction files, output XLM balance differences, missing/extra fields and conversion precision drift.
```bash
# Basic comparison between two data files
devkit compare before_data.json after_data.json

# Only compare balance-related fields, ignore timestamps and comments
devkit compare --filter xlm,stroops,balance old.toml new.toml

# Export structured JSON diff report for programmatic consumption
devkit compare --format json base.json target.json > diff_report.json
```
4. inspect
Deep-dive file parsing details; display raw XLM ↔ Stroops conversion results, min/max boundary values and precision metadata.
```bash
# Full inspection of all parsed data fields
devkit inspect tx_sample_001.json

# Restrict output to only unit conversion details (matches converter test logic)
devkit inspect --mode convert-only wallet.json

# Short summary mode: only show extreme XLM min/max values
devkit inspect --short batch_transactions.toml
```
5. convert
Native XLM / Stroops unit converter, reusing the core conversion utilities tested in utilities_converters.rs. Supports single value and bulk file conversion.
```bash
# Convert single XLM numeric value to stroops
devkit convert xlm-to-stroops 123.456789

# Convert single stroops value back to human-readable XLM
devkit convert stroops-to-xlm 999999999

# Bulk convert all XLM fields in a file to stroops, export new file
devkit convert file --target-unit stroops input_transactions.json -o tx_stroops_output.json
```
6. health
Run environment self-checks: verify CLI dependencies, Stellar constant integrity, file system permissions and converter library validity.
```bash
# Quick lightweight health check
devkit health

# Full deep scan: validate converter boundaries, directory access and config rules
devkit health --full --verbose

# Export machine-readable JSON health status report
devkit health --format json > health_check_results.json
```
7. metrics
Generate statistical summaries for bulk data: total XLM volume, positive/negative boundary extremes, conversion error counts and transaction counts.
```bash
# Calculate metrics for a single transaction file
devkit metrics single_transaction.json

# Scan entire directory and export human-readable Markdown summary
devkit metrics --dir ./tx_batch/ --report metrics_summary.md

# Only calculate statistics for XLM boundary values and conversion inaccuracies
devkit metrics --filter xlm-boundaries ./data/*.toml
```
8. completions
Generate shell auto-completion scripts for bash, zsh, fish and PowerShell to streamline CLI workflow.
```bash
# Generate bash completion script and install to standard path
devkit completions bash > ~/.bash_completions/devkit

# Generate zsh completion script
devkit completions zsh > ~/.zsh/completions/_devkit

# Generate Windows PowerShell completion script
devkit completions powershell > devkit_completion.ps1
```
9. version
Display version information for the CLI binary, devkit library and built-in Stellar network constants.
```bash
# Print concise version string
devkit version

# Verbose full version info: build timestamp, STROOPS_PER_XLM, min/max stroop limits
devkit version --verbose

# Raw version string output for shell script parsing
devkit version --raw
```
10. config
Manage global & local project configuration files; control default asset units, validation rules and output formatting.
```bash
# Show merged active config (global + local project settings)
devkit config show

# Set global default balance unit to stroops
devkit config set default_unit stroops

# Reset local directory-specific configuration
devkit config reset local

# Export full config backup to file
devkit config export ./devkit_config_backup.toml
```
