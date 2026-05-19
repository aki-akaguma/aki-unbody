# Code Review for aki-unbody

## Overview
`aki-unbody` is a well-implemented command-line utility for text processing, providing functionality similar to `head` and `tail` with an additional `inverse` mode. The project structure is clean, and it makes good use of custom libraries like `flood-tide` and `runnel`.

## Positive Aspects
- **Clean Architecture**: The separation of concerns between CLI parsing (`conf`), core logic (`run`), and utilities (`util`) is clear and effective.
- **Comprehensive Documentation**: `lib.rs` contains detailed documentation and usage examples, which is excellent for maintainability and user guidance.
- **Robust Error Handling**: The use of `anyhow` and the graceful handling of broken pipes demonstrate attention to detail in real-world CLI usage.
- **Extensive Testing**: The project includes a good set of integration tests covering various scenarios.

## Areas for Improvement

### 1. Performance Optimization in `run_normal`
In `src/run.rs`, `run_normal` uses a `Vec` as a buffer for the tail lines.
```rust
        tail_buffer.push(line_s);
        if tail_buffer.len() > max_tail {
            let _ = tail_buffer.remove(0);
        }
```
`tail_buffer.remove(0)` is an O(N) operation. For large values of `--tail`, this could become a bottleneck.
**Recommendation**: Use `std::collections::VecDeque` for the tail buffer to achieve O(1) removal from the front.

### 2. Memory Efficiency in `run_inverse`
In `src/run.rs`, `run_inverse` buffers all lines that are not part of the "head" into a `Vec` before removing the "tail" lines.
```rust
    for (curr_line_count, line) in sioe.pg_in().lines().enumerate() {
        let line_s = line?;
        if conf.opt_head.is_some() && curr_line_count < max_head {
            // nothing todo
        } else {
            body_buffer.push(line_s);
        }
    }
```
This means the entire "body" of the file is loaded into memory. For very large files, this could lead to excessive memory consumption or OOM (Out Of Memory) errors.
**Recommendation**: Implement a sliding window of size `max_tail`. Store lines in a `VecDeque` of size `max_tail`, and only print a line when it "falls out" of the window. This ensures memory usage is bounded by `max_tail` regardless of the file size.

### 3. Use of `#[rustfmt::skip]`
There are several instances of `#[rustfmt::skip]` in the codebase. While sometimes necessary for clarity in table-like structures, excessive use can lead to inconsistent formatting.
**Recommendation**: Ensure that `rustfmt` is used where possible to maintain a consistent style across the project.

### 4. Code Generation
The project uses `flood-tide-gen` for CLI code generation. This is a powerful approach but adds a layer of complexity for contributors who might not be familiar with the tool.
**Recommendation**: Ensure the `xtask` or build process for generating these files is well-documented.

## Conclusion
The codebase is of high quality and reflects a high level of technical expertise. Addressing the memory and performance concerns in `src/run.rs` will further improve its robustness and efficiency for large-scale text processing.

---
Review Date: 2026-05-20
Reviewer: Gemini CLI Agent
