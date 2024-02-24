# Line Cracker

Display warning when a line of code is too long.

## Installation

``` sh
cargo add line_cracker --dev
```

Or add the following to your `Cargo.toml`:

``` toml
[dev-dependencies]
line_cracker = "0.1"
```

## Usage

``` sh
cargo line-cracker <fine_name>
```

## Example

``` console
$ cargo line-cracker ./test/data/lorum_ipsum.txt
[WARN] .\test\data\lorum_ipsum.txt:10: expected 80 chars but got 101
[WARN] .\test\data\lorum_ipsum.txt:12: expected 80 chars but got 96
[WARN] .\test\data\lorum_ipsum.txt:14: expected 80 chars but got 87
[WARN] .\test\data\lorum_ipsum.txt:19: expected 80 chars but got 354
[WARN] .\test\data\lorum_ipsum.txt:26: expected 80 chars but got 150
[INFO] checked: .\test\data\lorum_ipsum.txt
```

## Todo

- [x] Escape non-text files
- [ ] Scan directory
- [ ] Scan multiple files
- [ ] Custom line length
- [ ] Colorize output
- [ ] ...
