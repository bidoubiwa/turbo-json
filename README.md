
<h1 align="center">TURBO-JSON</h1>

`turbo-json` takes as input a path to JSON files, and combines the valid JSON's in an array written in the standart output.

The memory usage will [not exceed 8kb](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html#method.new) per file as **read and write** are done in **a streaming manner**. Resulting in a very low memory usage and fast processing

<p align="center">
<img src="https://github.com/bidoubiwa/turbo-json/raw/main/assets/boat.png" width=300 />
</p>

### Example

![](https://github.com/bidoubiwa/turbo-json/raw/main/assets/json_combining.gif)


## How JSON files are combined

The input JSON files are combined and output as one JSON array.
When it encounters a JSON array as the root type of one of the input file, it will concatenate the array with the final output (see examples below).

### Example 1

Input files:
```json
{ "id": 1 } // file 1
```
```json
{ "id": 2 } // file 2
```

Output JSON:
```json
[
  { "id": 1 },
  { "id": 2 }
]
```

### Example 2

Input files:
```json
[ 1, 2, 3 ] // file 1
```

```json
{ "id": 1 } // file 2
```

Output JSON:
```json
[
  1,
  2,
  3,
  { "id": 1 }
]
```


## Features

- Read and write of input and output is done in streams.
- Files JSON's format are validated before combined.
- Validation of JSON files are multithreaded.


## CLI

### Installation

```bash
cargo install turbo-json
```

### Usage

```bash
turbo-json [files ...]
```

#### Example
```bash
turbo-json tests/misc/**/*.json
```
