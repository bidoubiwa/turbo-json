# WIP


## development


Watches files, output stdout in `output.json` and show clippy output in stdout.

```bash
cargo watch -x 'clippy && cargo run -- misc/**/* | tee output.json' --ignore output.json
```

