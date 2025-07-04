# crtime

A simple Rust CLI tool that print the creation datetime of a given file in local time.

## Installation

```bash
# Clone this repository
git clone https://github.com/shu-kitamura/crtime.git
cd crtime

# Build the binary
cargo build --release

# (Optional) Install globally
cargo install --path .
```

## Usage

```
Usage: crtime <FILEPATH>

Arguments:
  <FILEPATH>  The file path of the file to check the creation datetime

Options:
  -h, --help  Print help
```

## Example

```
$ crtime /path/to/src/main.rs 
2025/03/19 21:25:47
```

If your file system does not support creation times, the following error occurs
```
$ crtime /path/to/src/main.rs
creation time is not available for the filesystem
```
