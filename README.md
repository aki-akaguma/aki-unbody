# aki-unbody

*aki-unbody* is output first or last n lines, like a head and tail of linux command.

## Features

*aki-unbody*  is output first or last n lines, like a head and tail of linux command.

* command help

```text
aki-unbody --help
```

```text
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

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-unbody
```

2. you can build debian package:

```text
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
