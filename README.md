# aki-unbody

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

output first or last n lines, like a head and tail of linux command.

## Features

- output first or last n lines, like a head and tail of linux command.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-unbody --help
```

```
Usage:
  aki-unbody [options]

output first or last n lines, like a head and tail of linux command.

Options:
  -h, --head <num>      output the first <num> lines.
  -t, --tail <num>      output the last <num> lines.
  -i, --inverse         output the body, except for head and tail.

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Examples:
  Outputs first 2 lines:
    cat file1.txt | aki-unbody --head 2
  Outputs last 2 lines:
    cat file1.txt | aki-unbody --tail 2
  Outputs body, except for first 2 lines and last 2 lines:
    cat file1.txt | aki-unbody --head 2 --tail 2 --inverse
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-unbody
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

The input data used in this example looks like this:

```
cat file1.txt
```

result output:
```
LN:0001,text
LN:0002,text
LN:0003,text
LN:0004,text
LN:0005,text
LN:0006,text
```

### Example 1: output head

Outputs first 2 lines.

command line:
```
cat file1.txt | aki-unbody --head 2
```

result output:
```
LN:0001,text
LN:0002,text
```

### Example 2: output tail

Outputs last 2 lines.

command line:
```
cat file1.txt | aki-unbody --tail 2
```

result output:
```
LN:0005,text
LN:0006,text
```

### Example 3: output head and tail

Outputs first 2 lines and last 2 lines.

command line:
```
cat file1.txt | aki-unbody --head 2 --tail 2
```

result output:
```
LN:0001,text
LN:0002,text
LN:0005,text
LN:0006,text
```

### Example 4: output body, except for head and tail

Outputs body, except for first 2 lines and last 2 lines.

command line:
```
cat file1.txt | aki-unbody --head 2 --tail 2 --inverse
```

result output:
```
LN:0003,text
LN:0004,text
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-unbody/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-unbody.svg
[crate-link]: https://crates.io/crates/aki-unbody
[docs-image]: https://docs.rs/aki-unbody/badge.svg
[docs-link]: https://docs.rs/aki-unbody/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
