# SIMD Optimization Guide

This document describes how to enable and verify SIMD (Single Instruction Multiple Data) optimizations in the Hash Utility for maximum performance.

## Overview

The Hash Utility uses cryptographic hash implementations from the RustCrypto ecosystem and BLAKE3, which automatically leverage SIMD instructions when available. SIMD acceleration can provide 2-10x performance improvements depending on the algorithm and CPU capabilities.

## Supported SIMD Instruction Sets

### x86/x86_64 (Intel/AMD)
- **SSE2**: Baseline for x86_64, always available
- **SSE4.1**: Improved performance for some algorithms
- **AVX**: 256-bit vector operations
- **AVX2**: Enhanced 256-bit operations (recommended)
- **AVX-512**: 512-bit vector operations (highest performance)

### ARM/AArch64
- **NEON**: ARM's SIMD instruction set
- Available on most modern ARM processors (ARMv7 and later)

## Compilation Flags for Optimal Performance

### Option 1: Target Native CPU (Recommended for Local Use)

Build for your specific CPU with all available features:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

This enables all SIMD instructions supported by your CPU.

**Pros:**
- Maximum performance on your machine
- Automatic feature detection

**Cons:**
- Binary may not run on older CPUs
- Not portable across different machines

### Option 2: Target Specific CPU Features

Build with specific SIMD features:

```bash
# For AVX2 support (modern Intel/AMD CPUs from 2013+)
RUSTFLAGS="-C target-feature=+avx2" cargo build --release

# For AVX-512 support (high-end Intel CPUs from 2017+)
RUSTFLAGS="-C target-feature=+avx512f" cargo build --release

# For ARM NEON support
RUSTFLAGS="-C target-feature=+neon" cargo build --release
```

**Pros:**
- Control over which features are enabled
- Can target specific CPU generations

**Cons:**
- Requires knowledge of target CPU capabilities
- May crash on CPUs without the specified features

### Option 3: Portable Build with Runtime Detection

Build without specific CPU features (default):

```bash
cargo build --release
```

**Pros:**
- Binary runs on any CPU of the target architecture
- Hash crates use runtime CPU feature detection
- Automatically uses SIMD when available

**Cons:**
- Slightly lower performance than native builds
- Some algorithms may not use the fastest SIMD paths

## Algorithm-Specific SIMD Support

### BLAKE3
- **Best SIMD support** among all algorithms
- Uses runtime CPU feature detection automatically
- Performance with SIMD:
  - AVX-512: 3-5 GB/s
  - AVX2: 1-3 GB/s
  - SSE4.1: 500-1000 MB/s
  - NEON: 300-800 MB/s
  - Scalar: 50-200 MB/s

### SHA-2 Family (SHA-256, SHA-512)
- Uses SIMD through the `sha2` crate
- Runtime detection via `cpufeatures` crate
- Performance with SIMD:
  - SHA-256 with AVX2: 500-800 MB/s
  - SHA-512 with AVX2: 600-900 MB/s
  - Scalar: 100-300 MB/s

### BLAKE2
- SIMD support through the `blake2` crate
- Runtime detection available
- Performance with SIMD:
  - BLAKE2b with AVX2: 800-1200 MB/s
  - BLAKE2s with AVX2: 600-900 MB/s
  - Scalar: 200-400 MB/s

### SHA-3 Family
- Limited SIMD support in current implementations
- Primarily uses scalar implementations
- Performance: 200-400 MB/s

### MD5 and SHA-1
- Legacy algorithms with basic SIMD support
- Not recommended for security-critical applications
- Performance: 400-600 MB/s

## Verifying SIMD Support

### Run SIMD Verification Tests

The project includes comprehensive tests to verify SIMD support:

```bash
# Run all SIMD verification tests
cargo test --release --test simd_verification

# Run with output to see CPU features and performance
cargo test --release --test simd_verification -- --nocapture
```

### Run Benchmarks

Compare algorithm performance to verify SIMD is working:

```bash
# Build with native CPU features
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run benchmark command
./target/release/hash -b

# Or specify custom data size (in MB)
./target/release/hash -b 500
```

Expected results with SIMD enabled:
- BLAKE3 should be the fastest (1000+ MB/s on modern CPUs)
- BLAKE2 should be second fastest (800+ MB/s)
- SHA-256/512 should achieve 500+ MB/s
- SHA-3 should be slower (200-400 MB/s)

### Check CPU Features at Runtime

Run the CPU feature detection test:

```bash
cargo test --release test_cpu_feature_detection -- --nocapture
```

This will display which SIMD instruction sets are available on your CPU.

## Troubleshooting

### Low Performance

If benchmarks show unexpectedly low performance:

1. **Verify you're using release mode:**
   ```bash
   cargo build --release  # Not cargo build
   ```

2. **Check if SIMD features are enabled:**
   ```bash
   cargo test --release test_cpu_feature_detection -- --nocapture
   ```

3. **Try building with native CPU features:**
   ```bash
   RUSTFLAGS="-C target-cpu=native" cargo build --release
   ```

4. **Verify the binary is using SIMD:**
   ```bash
   # On Linux, check for SIMD instructions in the binary
   objdump -d target/release/hash | grep -i "avx\|sse"
   ```

### Binary Crashes with "Illegal Instruction"

This means the binary was compiled with CPU features not available on your system:

1. **Rebuild without target-cpu=native:**
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Or target a lower CPU feature set:**
   ```bash
   RUSTFLAGS="-C target-cpu=x86-64-v2" cargo build --release
   ```

### SIMD Not Available

If your CPU doesn't support SIMD instructions:

- All algorithms will automatically fall back to scalar implementations
- Performance will be lower but functionality is identical
- The application will work correctly on any CPU

## Platform-Specific Notes

### Linux
- SIMD features are automatically detected at runtime
- No special configuration needed
- Use `lscpu` to check CPU features

### macOS
- SIMD features are automatically detected
- Apple Silicon (M1/M2) uses NEON instructions
- Use `sysctl -a | grep cpu.features` to check features

### Windows
- SIMD features are automatically detected
- Use `wmic cpu get caption` to check CPU model
- May need Visual Studio Build Tools for compilation

### FreeBSD
- SIMD features are automatically detected
- Same behavior as Linux
- Use `sysctl hw.model` to check CPU

## Recommended Build Commands

### For Local Use (Maximum Performance)
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### For Distribution (Maximum Compatibility)
```bash
cargo build --release
```

### For Specific CPU Generation
```bash
# For modern CPUs (2013+) with AVX2
RUSTFLAGS="-C target-cpu=haswell" cargo build --release

# For newer CPUs (2017+) with AVX-512
RUSTFLAGS="-C target-cpu=skylake-avx512" cargo build --release
```

## Performance Expectations

With SIMD enabled on a modern CPU (AVX2 or better):

| Algorithm | Expected Throughput |
|-----------|-------------------|
| BLAKE3 | 1000-3000 MB/s |
| BLAKE2b | 800-1200 MB/s |
| SHA-512 | 600-900 MB/s |
| SHA-256 | 500-800 MB/s |
| SHA-3-256 | 200-400 MB/s |
| MD5 | 400-600 MB/s |

Without SIMD (scalar implementations):

| Algorithm | Expected Throughput |
|-----------|-------------------|
| BLAKE3 | 50-200 MB/s |
| BLAKE2b | 200-400 MB/s |
| SHA-512 | 150-300 MB/s |
| SHA-256 | 100-250 MB/s |
| SHA-3-256 | 100-200 MB/s |
| MD5 | 200-400 MB/s |

## References

- [BLAKE3 SIMD Documentation](https://github.com/BLAKE3-team/BLAKE3)
- [RustCrypto Project](https://github.com/RustCrypto)
- [Rust CPU Feature Detection](https://doc.rust-lang.org/std/arch/index.html)
- [RUSTFLAGS Documentation](https://doc.rust-lang.org/cargo/reference/environment-variables.html)

## Requirements Validation

This documentation addresses the following requirements:

- **Requirement 1.3**: SIMD instructions are utilized where hash algorithm implementations support hardware acceleration
- **Requirement 4.4**: SIMD instructions are utilized through runtime detection when target CPU supports them
- **Requirement 4.5**: Fallback to scalar implementations works without errors on systems without SIMD support
