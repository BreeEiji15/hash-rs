# Hash Utility

A high-performance, cross-platform cryptographic hash utility with SIMD optimization support.

## Features

- **Multiple Hash Algorithms**: MD5, SHA-1, SHA-2 family (SHA-224, SHA-256, SHA-384, SHA-512), SHA-3 family (SHA3-224, SHA3-256, SHA3-384, SHA3-512), BLAKE2b, BLAKE2s, and BLAKE3
- **SIMD Acceleration**: Automatic hardware acceleration using SSE, AVX, AVX2, AVX-512 (x86_64) and NEON (ARM)
- **Post-Quantum Algorithms**: Support for SHA-3 family algorithms
- **Directory Scanning**: Recursively hash all files in a directory
- **Hash Verification**: Compare current hashes against stored database
- **Benchmarking**: Measure hash algorithm performance on your hardware
- **Minimal Binary Size**: Optimized for size with LTO and stripping
- **Cross-Platform**: Works on Linux, macOS, Windows, and FreeBSD

## Quick Start

```bash
# Build the utility
cargo build --release

# Compute a hash
./target/release/hash hash -f myfile.txt -a sha256

# Scan a directory
./target/release/hash scan -d ./my_directory -a sha256 -o hashes.db

# Verify integrity
./target/release/hash verify -b hashes.db -d ./my_directory

# See all available algorithms
./target/release/hash list
```

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd hash-utility

# Build with release optimizations
cargo build --release

# The binary will be at target/release/hash
```

### Optimized Build (Maximum Performance)

For maximum performance on your specific CPU:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

See [SIMD_OPTIMIZATION.md](SIMD_OPTIMIZATION.md) for detailed compilation options.

## Usage

### Getting Help

```bash
# Display general help
hash --help
hash -h          # Short form (equivalent)

# Display help for a specific command
hash hash --help
hash scan --help
hash verify --help
hash benchmark --help
```

### Command-Line Flags

All commands support both short and long form flags:

| Short | Long | Description |
|-------|------|-------------|
| `-f` | `--file` | Specify input file |
| `-a` | `--algorithm` | Specify hash algorithm |
| `-o` | `--output` | Specify output file |
| `-d` | `--directory` | Specify directory |
| `-b` | `--database` | Specify database file |
| `-p` | `--parallel` | Enable parallel processing |
| `-s` | `--size` | Specify benchmark data size |
| `-h` | `--help` | Display help |
| `-V` | `--version` | Display version |

Both forms are equivalent and can be used interchangeably.

### Compute Hash for a File

```bash
# Single algorithm (defaults to SHA-256)
hash hash -f myfile.txt -a sha256

# Multiple algorithms in one pass (more efficient than running separately)
hash hash -f myfile.txt -a sha256 -a blake3 -a sha3-256

# Save output to file instead of displaying on screen
hash hash -f myfile.txt -a sha256 -o hashes.txt

# Using long-form flags
hash hash --file myfile.txt --algorithm sha256 --output hashes.txt
```

**Output format:**
```
<hash_hex>  <filepath>
```

Example:
```
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  myfile.txt
```

### Scan Directory

Recursively hash all files in a directory:

```bash
# Basic directory scan
hash scan -d /path/to/directory -a sha256 -o hashes.db

# Use parallel processing for faster scanning (recommended for large directories)
hash scan -d /path/to/directory -a sha256 -o hashes.db --parallel

# Scan current directory
hash scan -d . -a blake3 -o checksums.txt

# Using long-form flags
hash scan --directory /path/to/directory --algorithm sha256 --output hashes.db --parallel
```

**Progress information:**
During scanning, you'll see:
- Number of files processed
- Number of files that failed (e.g., permission errors)
- Total bytes processed
- Time elapsed

### Verify Directory

Compare current hashes against a stored database:

```bash
# Verify directory integrity
hash verify -b hashes.db -d /path/to/directory

# Using long-form flags
hash verify --database hashes.db --directory /path/to/directory
```

**Verification report includes:**
- **Matches**: Files with unchanged hashes (integrity verified)
- **Mismatches**: Files with changed hashes (modified or corrupted)
- **Missing**: Files in database but not in filesystem (deleted)
- **New**: Files in filesystem but not in database (added)

Example output:
```
Verification Report:
  Matches: 150 files
  Mismatches: 2 files
    - /path/to/modified.txt
      Expected: abc123...
      Actual:   def456...
  Missing: 1 file
    - /path/to/deleted.txt
  New: 3 files
    - /path/to/newfile1.txt
    - /path/to/newfile2.txt
```

### Benchmark Algorithms

Test hash algorithm performance on your hardware:

```bash
# Default benchmark (100 MB test data)
hash benchmark

# Custom data size (in MB)
hash benchmark --size 500

# Short form
hash benchmark -s 1000
```

**Output shows:**
- Algorithm name
- Throughput in MB/s
- Relative performance comparison

### List Available Algorithms

```bash
hash list
```

This displays all supported algorithms with:
- Algorithm name
- Output size in bits
- Post-quantum resistance status

Example output:
```
Available Hash Algorithms:

Algorithm            Output Bits   Post-Quantum
--------------------------------------------------
md5                          128             No
sha1                         160             No
sha224                       224             No
sha256                       256             No
sha384                       384             No
sha512                       512             No
sha3-224                     224            Yes
sha3-256                     256            Yes
sha3-384                     384            Yes
sha3-512                     512            Yes
blake2b                      512             No
blake2s                      256             No
blake3                       256             No
```

## Common Use Cases

### Verify Downloaded File Integrity

```bash
# Download a file and its published hash
# Compute the hash and compare
hash hash -f downloaded-file.iso -a sha256

# Compare the output with the published hash
```

### Create Backup Verification Database

```bash
# Before backup: create hash database
hash scan -d /important/data -a sha256 -o backup-hashes.db --parallel

# After restore: verify integrity
hash verify -b backup-hashes.db -d /restored/data
```

### Monitor Directory for Changes

```bash
# Create baseline
hash scan -d /etc/config -a sha256 -o baseline.db

# Later, check for changes
hash verify -b baseline.db -d /etc/config
```

### Compare Two Directories

```bash
# Hash first directory
hash scan -d /path/to/dir1 -a sha256 -o dir1.db

# Hash second directory
hash scan -d /path/to/dir2 -a sha256 -o dir2.db

# Compare the database files (using standard tools)
diff dir1.db dir2.db
```

### Forensic Analysis

```bash
# Use post-quantum resistant algorithm for long-term integrity
hash scan -d /evidence -a sha3-256 -o evidence-hashes.db

# Multiple algorithms for redundancy
hash hash -f critical-file.bin -a sha256 -a sha3-256 -a blake3
```

## Performance

With SIMD enabled on modern hardware (AVX2 or better):

| Algorithm | Typical Throughput |
|-----------|-------------------|
| BLAKE3 | 1000-3000 MB/s |
| BLAKE2b | 800-1200 MB/s |
| SHA-512 | 600-900 MB/s |
| SHA-256 | 500-800 MB/s |
| SHA-3-256 | 200-400 MB/s |

Actual performance depends on your CPU and compilation flags. Run `hash benchmark` to test on your system.

**Performance Tips:**
- Use `--parallel` flag for scanning large directories
- Compile with `RUSTFLAGS="-C target-cpu=native"` for maximum performance
- BLAKE3 is typically the fastest algorithm
- SHA-512 is often faster than SHA-256 on 64-bit systems

## SIMD Optimization

The Hash Utility automatically uses SIMD instructions when available:

- **x86_64**: SSE2, SSE4.1, AVX, AVX2, AVX-512
- **ARM/AArch64**: NEON

### Verifying SIMD Support

Run the SIMD verification tests:

```bash
cargo test --release --test simd_verification -- --nocapture
```

This will:
- Test that BLAKE3 uses SIMD when available
- Verify scalar fallback works correctly
- Display available CPU features
- Show performance comparisons

For detailed information about SIMD optimization, see [SIMD_OPTIMIZATION.md](SIMD_OPTIMIZATION.md).

## Database Format

The hash database uses a simple plain text format compatible with standard checksum tools:

```
<hash>  <filepath>
```

Example:
```
d41d8cd98f00b204e9800998ecf8427e  ./empty.txt
5d41402abc4b2a76b9719d911017c592  ./hello.txt
```

## Post-Quantum Algorithms

The following algorithms are considered post-quantum resistant:

- SHA3-224
- SHA3-256
- SHA3-384
- SHA3-512

Use `hash list` to see which algorithms are marked as post-quantum.

## Cross-Platform Support

The Hash Utility works on:

- **Linux** (x86_64, ARM64)
- **macOS** (Intel, Apple Silicon)
- **Windows** (x86_64)
- **FreeBSD** (x86_64)

Path handling is automatically adapted for each platform.

## Troubleshooting

### "Unsupported algorithm" Error

```bash
# List all available algorithms
hash list

# Use exact algorithm name from the list
hash hash -f myfile.txt -a sha256  # Correct
hash hash -f myfile.txt -a SHA256  # May not work (case-sensitive)
```

### Permission Errors During Scan

The utility will log permission errors but continue scanning other files:

```bash
# Run with appropriate permissions
sudo hash scan -d /protected/directory -a sha256 -o hashes.db
```

### Verification Shows Many Mismatches

Possible causes:
1. **Different algorithm used**: Ensure you use the same algorithm for scan and verify
2. **Files were actually modified**: This is expected behavior
3. **Path differences**: Ensure you're verifying from the same base directory

```bash
# Check which algorithm was used in the database
head -1 hashes.db  # Look at hash length to identify algorithm
```

### Slow Performance

```bash
# Enable parallel processing
hash scan -d /large/directory -a sha256 -o hashes.db --parallel

# Use a faster algorithm
hash scan -d /large/directory -a blake3 -o hashes.db --parallel

# Compile with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Output Not Appearing

If using `-o` flag, output goes to file, not stdout:

```bash
# This writes to file (nothing on screen)
hash hash -f myfile.txt -a sha256 -o output.txt

# This displays on screen
hash hash -f myfile.txt -a sha256
```

### Binary Size Too Large

```bash
# Ensure you're building in release mode
cargo build --release

# Check the binary size
ls -lh target/release/hash

# Strip additional symbols (if not already done)
strip target/release/hash
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --release

# Run SIMD verification tests with output
cargo test --release --test simd_verification -- --nocapture

# Run unit tests
cargo test --release --lib
```

### Building for Distribution

```bash
# Portable build (works on any CPU of target architecture)
cargo build --release

# Optimized for specific CPU generation
RUSTFLAGS="-C target-cpu=haswell" cargo build --release  # AVX2 support
```

## Requirements

- Rust 1.70 or later
- No runtime dependencies beyond the standard library

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

This project uses hash implementations from:
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic hash functions
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - High-performance cryptographic hash
