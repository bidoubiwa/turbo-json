# Contributing

## Run project in Development

### Installation

```bash
git clone git@github.com:bidoubiwa/turbo-json.git
cd turbo-json
```

Make `turbo-json` a global command.
```bash
cargo install --path .
```

### Usage

The following will fetch all files inside misc recursively and output it's combined JSON.

With a **local run**:
```bash
cargo run --release -- [file ...]
```

**example:**
```bash
cargo run --release -- tests/misc/**/*
```

With the **global** install:

```bash
turbo-json [files ...]
```

**example:**
```bash
turbo-json misc/**/*
```


## Development utilities

Watches files, output stdout in `output.json` and show clippy output in stdout.

```bash
cargo watch -x 'clippy && cargo run -- tests/misc/**/* | tee output.json' --ignore output.json
```

