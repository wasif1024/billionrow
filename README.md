# 1️⃣🐝🏎️ The One Billion Row Challenge (Rust Edition)

A Rust implementation of [The One Billion Row Challenge](https://github.com/gunnarmorling/1brc) -- A fun exploration of how quickly 1B rows from a text file can be aggregated.

## The Challenge

Write a program that reads a text file containing temperature measurements and computes the min, mean, and max temperature per weather station.

The input file has the following format:
- One measurement per line
- Each line has the format: `<string:station name>;<float:temperature>`
- Station names are arbitrary UTF-8 strings (guaranteed to contain no `;` or `\n` characters)
- Temperatures are floating point numbers with one decimal place
- Example: `Hamburg;12.0\nBulawayo;8.9\nPalembang;38.8\n`

The output must be printed to stdout with the following format:
- One station per line
- Format: `<station name>=<min>/<mean>/<max>`
- Stations must be sorted alphabetically
- Example: `{Abha=-23.0/18.0/59.2,Abidjan=-16.2/26.0/67.3,...}`

## Requirements

- **Rust**: This implementation requires Rust (stable or nightly)
- **Unix-like OS**: The implementation uses memory-mapped files via `mmap`, which requires a Unix-like system (Linux, macOS, etc.)
- **Data file**: Place the `measurements.txt` file in the `./data/` directory

## Building

Build the release version with optimizations:

```bash
cargo build --release
```

The optimized binary will be located at `target/release/billionrow`.

## Running

Make sure you have the `measurements.txt` file in the `./data/` directory, then run:

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/billionrow
```

The program will read from `./data/measurements.txt` and output the aggregated results to stdout.

## Implementation Details

This Rust implementation uses several performance optimizations:

- **Memory-mapped files**: Uses `mmap` to map the entire file into memory for zero-copy reading
- **Manual parsing**: Custom integer parsing algorithm that avoids string-to-number conversion overhead
- **Integer arithmetic**: Uses fixed-point arithmetic (i16) to represent temperatures, avoiding floating-point operations during aggregation
- **Efficient data structures**: Uses `HashMap` for fast lookups during aggregation and `BTreeMap` for sorted output
- **Optimized compilation**: Release profile configured with LTO, native CPU optimizations, and panic=abort

## Performance

Performance results will vary based on:
- Hardware (CPU, cores, RAM)
- Operating system
- File system (RAM disk vs. disk)
- Compiler optimizations

To benchmark your system:

```bash
time cargo run --release
```

## Testing

To verify correctness, compare the output against a reference implementation. The output format should match:

```
{Station1=min/mean/max,Station2=min/mean/max,...}
```

## Project Structure

```
billionrow/
├── Cargo.toml          # Rust project configuration
├── src/
│   └── main.rs         # Main implementation
└── data/
    └── measurements.txt # Input data file (not included)
```

## Dependencies

- `libc`: For low-level system calls (mmap, madvise)

## License

This implementation is inspired by [The One Billion Row Challenge](https://github.com/gunnarmorling/1brc) by Gunnar Morling, which is licensed under Apache License 2.0.

## References

- Original Challenge: https://github.com/gunnarmorling/1brc
- Challenge Blog Post: https://www.morling.dev/blog/one-billion-row-challenge/

## Notes

- The implementation uses `unsafe` Rust for memory-mapped file access and UTF-8 string conversion, which is safe in this context given the known data format
- The code is optimized for performance and may sacrifice some readability for speed
- This is a learning project focused on exploring Rust's performance capabilities
