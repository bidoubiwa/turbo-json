
<h1 align="center">json-boat</h1>

<p align="center">
<img src="assets/boat.png" width=300 />
</p>

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
- [x] API
  - [x] wrapper thatdoes the same as CLI but library
- [x] Reader
  - [x] Reads all input files in arguments order
- [x] Validator
  - [x] Validate input files during stream read
- [x] Writer
  - [x] Write input files into output file after validating
- [x] Computer `,` `[` `]`
  - [ ] Add needed symbols to create valid output json

