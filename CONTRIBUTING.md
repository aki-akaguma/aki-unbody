# Contributing to aki-unbody

Thank you for your interest in contributing to `aki-unbody`! This document provides guidelines and instructions for developing and contributing to this project.

## Development Environment

To set up your development environment, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (Minimum Supported Rust Version: 1.68.0)
- `make` (optional, for using the provided Makefile)

## Development Workflow

### 1. Updating CLI Options

The Command Line Interface (CLI) code is partially generated using `flood-tide-gen`. **Do not edit the generated files directly.**

If you need to change the CLI options:
1.  Edit the option definition file: `xtask/src/aki-unbody-cmd.txt`.
2.  If the change involves type changes or new parameters, you may also need to update the fixup logic in `xtask/src/gen_src_cmd.rs`.
3.  Run the generation command:
    ```bash
    make gen-src-cmd
    ```
    (This runs `cargo xtask gen-src-cmd gen-src-cmd` under the hood.)

The generated files are:
- `src/conf/cmd.help.rs.txt`
- `src/conf/cmd.match.rs.txt`

### 2. Formatting

We use `rustfmt` to maintain a consistent code style. Please format your code before committing:
```bash
make fmt
```

### 3. Linting

Run `clippy` to check for common mistakes and improve your code:
```bash
make clippy
```

### 4. Testing

Ensure all tests pass before submitting your changes:
```bash
make test
```

### 5. Updating Documentation

The `README.md` is generated from `README.tpl` and the documentation comments in `src/lib.rs`. If you change these, update the `README.md`:
```bash
make readme
```

## Pull Request Process

1.  Fork the repository and create your branch from `main`.
2.  Ensure your code follows the project's style and passes all tests.
3.  Update the `CHANGELOG.md` in the `[Unreleased]` section with a brief description of your changes.
4.  Submit a Pull Request with a clear description of the problem and your solution.
