# JSONBOAT

Streams json files into on big json. Reading and writing done with streaming. Json is validated while streaming as well.

## Usage

The following will fetch all files inside misc recursively.

```bash
cargo run -- misc/**/*
```

## TODO
- [ ] CLI
  - [x] input json lists
  - [ ] bonus: stream read and write buffer size
- [ ] API
  - [ ] wrapper thatdoes the same as CLI but library
- [ ] Reader
  - [ ] Reads all input files in arguments order
- [ ] Validator
  - [ ] Validate input files during stream read
- [ ] Writer
  - [ ] Write input files into output file after validating
- [ ] Computer `,` `[` `]`
  - [ ] Add needed symbols to create valid output json

