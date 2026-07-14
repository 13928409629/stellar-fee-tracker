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
```bash

2. repair
Auto-fix invalid data flagged by validate: normalize XLM to 6 decimal places, resolve out-of-range stroops values and fix unit mismatches.
```bash
# In-place repair (overwrite original file)
devkit repair ./wallets/user_wallet.json

# Generate fixed output file without modifying source
devkit repair --output ./wallets/user_wallet_fixed.json ./wallets/user_wallet.json

# Batch repair all files in directory, append .fixed suffix to outputs
devkit repair --dir ./transactions/ --suffix .fixed --ext json
```bash
3. compare
Compare two config / transaction files, output XLM balance differences, missing/extra fields and conversion precision drift.
```bash
# Basic comparison between two data files
devkit compare before_data.json after_data.json

# Only compare balance-related fields, ignore timestamps and comments
devkit compare --filter xlm,stroops,balance old.toml new.toml

# Export structured JSON diff report for programmatic consumption
devkit compare --format json base.json target.json > diff_report.json
```bash
4. inspect
Deep-dive file parsing details; display raw XLM ↔ Stroops conversion results, min/max boundary values and precision metadata.
```bash
# Full inspection of all parsed data fields
devkit inspect tx_sample_001.json

# Restrict output to only unit conversion details (matches converter test logic)
devkit inspect --mode convert-only wallet.json

# Short summary mode: only show extreme XLM min/max values
devkit inspect --short batch_transactions.toml
```bash
5. convert
Native XLM / Stroops unit converter, reusing the core conversion utilities tested in utilities_converters.rs. Supports single value and bulk file conversion.
```bash
# Convert single XLM numeric value to stroops
devkit convert xlm-to-stroops 123.456789

# Convert single stroops value back to human-readable XLM
devkit convert stroops-to-xlm 999999999

# Bulk convert all XLM fields in a file to stroops, export new file
devkit convert file --target-unit stroops input_transactions.json -o tx_stroops_output.json
```bash
6. health
Run environment self-checks: verify CLI dependencies, Stellar constant integrity, file system permissions and converter library validity.
```bash
# Quick lightweight health check
devkit health

# Full deep scan: validate converter boundaries, directory access and config rules
devkit health --full --verbose

# Export machine-readable JSON health status report
devkit health --format json > health_check_results.json
```bash
7. metrics
Generate statistical summaries for bulk data: total XLM volume, positive/negative boundary extremes, conversion error counts and transaction counts.
```bash
# Calculate metrics for a single transaction file
devkit metrics single_transaction.json

# Scan entire directory and export human-readable Markdown summary
devkit metrics --dir ./tx_batch/ --report metrics_summary.md

# Only calculate statistics for XLM boundary values and conversion inaccuracies
devkit metrics --filter xlm-boundaries ./data/*.toml
```bash
8. completions
Generate shell auto-completion scripts for bash, zsh, fish and PowerShell to streamline CLI workflow.
```bash
# Generate bash completion script and install to standard path
devkit completions bash > ~/.bash_completions/devkit

# Generate zsh completion script
devkit completions zsh > ~/.zsh/completions/_devkit

# Generate Windows PowerShell completion script
devkit completions powershell > devkit_completion.ps1
```bash
9. version
Display version information for the CLI binary, devkit library and built-in Stellar network constants.
```bash
# Print concise version string
devkit version

# Verbose full version info: build timestamp, STROOPS_PER_XLM, min/max stroop limits
devkit version --verbose

# Raw version string output for shell script parsing
devkit version --raw
```bash
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
```bash
### Global Shared Flags (Available for all subcommands)
```bash
devkit -v/--verbose          # Enable detailed debug logs
devkit -q/--quiet             # Quiet mode, only output error messages
devkit --log-level [debug|info|warn|error]  # Custom log level
devkit -c/--config <PATH>    # Load custom configuration file
validate
Validate wallet, transaction and configuration files, detect XLM precision errors, out-of-bound stroops values, invalid decimal digits and malformed structures.
bash
运行
# Basic single file validation
devkit validate ./wallets/user_wallet.json
# Strict validation with detailed error logs
devkit validate --strict --verbose ./transactions/batch.toml
# Batch validate all files under a directory
devkit validate --dir ./wallets/ --ext json,toml
# Only validate XLM and stroops balance fields
devkit validate --filter xlm,stroops ./ledger_records.json
repair
Auto-fix invalid data reported by validate, normalize XLM to standard 6 decimal places, fix out-of-range stroops and unit mismatch issues.
bash
运行
# In-place repair, overwrite original file
devkit repair ./wallets/user_wallet.json
# Output fixed data to a new file without modifying source
devkit repair --output ./wallets/user_wallet_fixed.json ./wallets/user_wallet.json
# Batch repair all files and add .fixed suffix to outputs
devkit repair --dir ./transactions/ --suffix .fixed --ext json
compare
Compare two config or transaction files, output XLM balance differences, missing fields and conversion precision deviation.
bash
运行
# Basic comparison of two data files
devkit compare before_data.json after_data.json
# Only compare balance-related fields, ignore timestamps and comments
devkit compare --filter xlm,stroops,balance old.toml new.toml
# Export structured JSON diff report for automation
devkit compare --format json base.json target.json > diff_report.json
inspect
Deeply parse file data, display complete XLM ↔ Stroops conversion results, boundary values and precision diagnostic info.
bash
运行
# Full inspection of all parsed fields
devkit inspect tx_sample_001.json
# Only show unit conversion related details
devkit inspect --mode convert-only wallet.json
# Short summary mode, only output min/max XLM boundary values
devkit inspect --short batch_transactions.toml
convert
XLM / Stroops unit converter, reuse core conversion logic tested in utilities_converters.rs. Support single value conversion and bulk file conversion.
bash
运行
# Convert single XLM value to stroops
devkit convert xlm-to-stroops 123.456789
# Convert stroops back to human-readable XLM
devkit convert stroops-to-xlm 999999999
# Bulk convert all XLM fields in file to stroops and export new file
devkit convert file --target-unit stroops input_transactions.json -o tx_stroops_output.json
health
Run environment self-inspection, verify CLI dependencies, Stellar constant integrity, file permissions and converter library availability.
bash
运行
# Quick lightweight health check
devkit health
# Full deep scan with converter boundary validation
devkit health --full --verbose
# Export machine-readable JSON health report
devkit health --format json > health_check_results.json
metrics
Generate statistical summary for bulk data, including total XLM amount, positive/negative boundary values and conversion error statistics.
bash
运行
# Calculate metrics for single transaction file
devkit metrics single_transaction.json
# Scan entire directory and export Markdown summary report
devkit metrics --dir ./tx_batch/ --report metrics_summary.md
# Only count XLM boundary and conversion error metrics
devkit metrics --filter xlm-boundaries ./data/*.toml
completions
Generate shell auto-completion scripts for bash, zsh, fish and PowerShell to accelerate local development.
bash
运行
# Generate bash completion script
devkit completions bash > ~/.bash_completions/devkit
# Generate zsh completion script
devkit completions zsh > ~/.zsh/completions/_devkit
# Generate PowerShell completion script for Windows
devkit completions powershell > devkit_completion.ps1
version
Print version information of CLI binary, devkit library and built-in Stellar network constants.
bash
运行
# Print concise version string
devkit version
# Print full verbose version details
devkit version --verbose
# Output raw version string for shell script parsing
devkit version --raw
config
Manage global and local project configuration, control default asset unit, validation rules and output format settings.
bash
运行
# Show merged active config (global + local project settings)
devkit config show
# Set global default balance unit to stroops
devkit config set default_unit stroops
# Reset local directory configuration to default
devkit config reset local
# Export full config backup to file
devkit config export ./devkit_config_backup.toml
# stellar-devkit

Developer toolkit for the Stellar Fee Tracker. Provides utilities for testing, mocking, and simulating Stellar network behaviour without hitting live infrastructure.

## Scope

`stellar-devkit` is a standalone testing and simulation package. It must not import from `stellar-core` or any live-network crate. All functionality is self-contained and intended for use in `[dev-dependencies]` only.

## Boundary Rules

- No imports from `packages/core`
- No live Horizon API calls
- No database connections
- All external I/O must be injectable or mockable

## Modules

| Module | Description |
|---|---|
| `harness` | Mock Horizon server and pre-built fee scenario fixtures |
| `harness::scenarios` | JSON scenario files and runtime loader |
| `simulation` | Fee models, network-load generators, congestion predictors |
| `analysis` | Percentile stats, spike classification, rolling window |
| `cli` | Replay, export, and benchmark CLI stubs |
| `types` | Shared types: `FeeRecord`, `Scenario`, `SimResult` |
| `error` | `DevkitError` unified error enum |

## Simulation

The `simulation` module provides fee modelling, network-load generation, and congestion prediction without any live-network dependencies.

### `FeeModelConfig` fields

| Field | Type | Default | Description |
|---|---|---|---|
| `base_fee` | `u64` | `100` | Base fee in stroops |
| `spike_probability` | `f64` | `0.05` | Probability that any given ledger is a spike (0.0–1.0) |
| `spike_multiplier` | `u64` | `10` | Multiplier applied to `base_fee` during a spike |
| `ledger_interval_secs` | `u64` | `5` | Seconds between simulated ledgers |
| `ledger_count` | `u64` | `100` | Number of ledgers to generate per `run()` call |
| `seed` | `Option<u64>` | `None` | RNG seed for reproducible output |
| `noise_factor` | `f64` | `0.0` | Gaussian noise stddev as a fraction of `base_fee` |

### `NetworkLoadConfig` fields

| Field | Type | Default | Description |
|---|---|---|---|
| `min_tx` | `u64` | `10` | Minimum transactions per ledger |
| `max_tx` | `u64` | `1000` | Maximum transactions per ledger |
| `ledger_capacity` | `u64` | `1000` | Maximum tx capacity per ledger |
| `ledger_interval_ms` | `u64` | `5000` | Time between ledger closes in ms |
| `seed` | `Option<u64>` | `None` | RNG seed for reproducibility |

### Example usage

```rust
use stellar_devkit::simulation::fee_model::{FeeModel, FeeModelConfig};
use stellar_devkit::simulation::network_load::{NetworkLoad, NetworkLoadConfig};
use stellar_devkit::simulation::congestion_predictor::{CongestionPredictor, CongestionInput, congestion_label};

// Configure a fee scenario
let fee_cfg = FeeModelConfig {
    base_fee: 100,
    spike_probability: 0.1,
    spike_multiplier: 5,
    seed: Some(42),
    ..FeeModelConfig::default()
};

// Generate fee points
let points = FeeModel::run(&fee_cfg);
println!("Generated {} fee points", points.len());

// Configure network load
let load_cfg = NetworkLoadConfig {
    min_tx: 50,
    max_tx: 800,
    ledger_capacity: 1000,
    seed: Some(7),
    ..NetworkLoadConfig::default()
};
let mut load = NetworkLoad::new(load_cfg);
let ledgers = load.simulate(10);

// Predict congestion
let label = congestion_label(&CongestionInput {
    recent_fee_window: 250.0,
    capacity_usage: 0.75,
    spike_count: 3,
});
println!("Congestion: {:?}", label);
```

### Output format (`FeePoint`)

Each `FeePoint` represents a single simulated ledger:

| Field | Type | Description |
|---|---|---|
| `timestamp` | `u64` | Simulated Unix timestamp (seconds) |
| `fee` | `u64` | Fee in stroops for this ledger |
| `ledger` | `u64` | Ledger sequence number (1-based) |
| `is_spike` | `bool` | Whether this ledger was a spike |

### CSV export

Fee points can be exported to CSV via the CLI:

```bash
cargo run --bin devkit -- export ./fees.db --output fees.csv
```

The CSV format matches the `FeePoint` shape:

```
timestamp,fee,ledger,is_spike
1700000000,100,1,false
1700000005,500,2,true
1700000010,110,3,false
```

For programmatic export, serialise `FeePoint` slices directly:

```rust
use stellar_devkit::simulation::fee_model::{FeeModel, FeeModelConfig, FeeCurve};

let points = FeeModel::run(&FeeModelConfig::default());
let json = FeeCurve::fee_points_to_json(&points, 100)?;
println!("{}", json);
```

## Running

```bash
# Run all devkit tests
cargo test -p stellar-devkit

# Run a specific test file
cargo test -p stellar-devkit --test harness_congested
```

## Mock Horizon Server

The harness exposes canned `GET /fee_stats` payloads through `HorizonMock` and the JSON fixtures in `src/harness/scenarios/`.

```bash
# Start with the baseline fixture
cargo test -p stellar-devkit --test harness_normal -- --nocapture

# Swap to a higher-pressure fixture
cargo test -p stellar-devkit --test harness_congested -- --nocapture
```

Scenario flags map directly to the fixture you load in your test setup:

- `normal` for a low-fee baseline
- `congested` for sustained high-fee demand
- `spike` for a sudden short-lived fee jump
- `recovery` for a return from congestion toward baseline

```rust
use std::path::Path;

use stellar_devkit::harness::{
    horizon_mock::HorizonMock,
    scenarios::load_from_file,
};

let payload = load_from_file(Path::new("src/harness/scenarios/spike.json"))?;
let mock = HorizonMock::new(payload);
assert!(mock.fee_stats_payload().contains("\"scenario\": \"spike\""));
```

## CLI

The devkit ships with a set of subcommands for driving scenarios from the command line.

### Usage

```bash
devkit <SUBCOMMAND> [OPTIONS]
```

### Subcommands

| Subcommand | Description |
|---|---|
| `replay` | Replay recorded fee scenarios from a SQLite database |
| `export` | Export fee data to CSV |
| `benchmark` | Run performance benchmarks against the fee pipeline |
| `mock` | Serve mock Horizon `/fee_stats` responses |
| `simulate` | Run a network-load simulation and print results |

### Examples

```bash
# Replay fee records from a local SQLite file
devkit replay ./fees.db

# Export fee data to CSV
devkit export ./fees.db --output fees.csv

# Run benchmarks
devkit benchmark --samples 1000

# Start the mock server
devkit mock --port 8080 --scenario spike
```

## Adding to Your Crate

```toml
[dev-dependencies]
stellar-devkit = { path = "../devkit" }
```

## Benchmarks

Baseline results measured on reference hardware (Apple M-series, single-core, `cargo bench`):

| Benchmark | Input | Mean | Std Dev |
|---|---|---|---|
| `fee_model/run_100` | 100 ledgers, seeded | ~12 µs | ±0.3 µs |
| `fee_model/run_1000` | 1 000 ledgers, seeded | ~115 µs | ±2 µs |
| `percentile/nearest_rank_1k` | 1 000 sorted values, p50 | ~1.8 µs | ±0.05 µs |
| `rolling_window/push_1k` | 1 000 pushes, window=100 | ~900 ns | ±20 ns |

### Running benchmarks locally

```bash
cargo bench --manifest-path packages/devkit/Cargo.toml
```

HTML reports are saved to `packages/devkit/target/criterion/`.

### CI benchmarks

Benchmarks compile and run on every PR touching `packages/devkit/` via the [Devkit Benchmarks](.github/workflows/devkit-bench.yml) workflow. Results are posted to the GitHub Actions step summary.
```toml
[dev-dependencies]
stellar-devkit = { path = "../devkit" }
```
