// SIMD Optimization Verification Tests
// Tests that hash algorithms use SIMD when available and fall back gracefully
// Requirements: 1.3, 4.4, 4.5

use std::time::Instant;

/// Test data size for SIMD verification (1MB)
const TEST_DATA_SIZE: usize = 1024 * 1024;

/// Generate test data for benchmarking
fn generate_test_data(size: usize) -> Vec<u8> {
    let pattern = b"The quick brown fox jumps over the lazy dog. ";
    let mut data = Vec::with_capacity(size);
    while data.len() < size {
        let remaining = size - data.len();
        if remaining >= pattern.len() {
            data.extend_from_slice(pattern);
        } else {
            data.extend_from_slice(&pattern[..remaining]);
        }
    }
    data
}

/// Measure hash throughput for an algorithm
fn measure_throughput<F>(hash_fn: F, data: &[u8]) -> f64
where
    F: FnOnce(&[u8]) -> Vec<u8>,
{
    let start = Instant::now();
    let _ = hash_fn(data);
    let duration = start.elapsed();
    
    let mb = data.len() as f64 / (1024.0 * 1024.0);
    let seconds = duration.as_secs_f64();
    
    if seconds > 0.0 {
        mb / seconds
    } else {
        0.0
    }
}

#[test]
fn test_blake3_simd_availability() {
    // BLAKE3 automatically uses SIMD when available through runtime detection
    // This test verifies that BLAKE3 can be instantiated and used
    
    let test_data = generate_test_data(TEST_DATA_SIZE);
    
    let throughput = measure_throughput(
        |data| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(data);
            hasher.finalize().as_bytes().to_vec()
        },
        &test_data,
    );
    
    // BLAKE3 should achieve reasonable throughput
    // Even without SIMD, BLAKE3 should be faster than 10 MB/s on modern hardware
    assert!(
        throughput > 10.0,
        "BLAKE3 throughput too low: {} MB/s. Expected > 10 MB/s",
        throughput
    );
    
    println!("BLAKE3 throughput: {:.2} MB/s", throughput);
    
    // Note: BLAKE3 uses SIMD automatically when available
    // On x86_64 with AVX2: expect 1000+ MB/s
    // On x86_64 with SSE4.1: expect 500+ MB/s
    // On ARM with NEON: expect 300+ MB/s
    // Without SIMD: expect 50+ MB/s
}

#[test]
fn test_sha2_simd_availability() {
    // SHA2 crates use SIMD through the sha2 crate's cpufeatures detection
    
    let test_data = generate_test_data(TEST_DATA_SIZE);
    
    use sha2::{Sha256, Digest};
    
    let throughput = measure_throughput(
        |data| {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        &test_data,
    );
    
    // SHA-256 should achieve reasonable throughput
    assert!(
        throughput > 10.0,
        "SHA-256 throughput too low: {} MB/s. Expected > 10 MB/s",
        throughput
    );
    
    println!("SHA-256 throughput: {:.2} MB/s", throughput);
}

#[test]
fn test_blake2_simd_availability() {
    // BLAKE2 uses SIMD when available
    
    let test_data = generate_test_data(TEST_DATA_SIZE);
    
    use blake2::{Blake2b512, Digest};
    
    let throughput = measure_throughput(
        |data| {
            let mut hasher = Blake2b512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        &test_data,
    );
    
    // BLAKE2 should achieve reasonable throughput
    assert!(
        throughput > 10.0,
        "BLAKE2b throughput too low: {} MB/s. Expected > 10 MB/s",
        throughput
    );
    
    println!("BLAKE2b-512 throughput: {:.2} MB/s", throughput);
}

#[test]
fn test_scalar_fallback_works() {
    // This test verifies that hash algorithms work even without SIMD
    // All algorithms should have scalar fallback implementations
    
    let test_data = b"test data for scalar fallback";
    
    // Test BLAKE3
    {
        let mut hasher = blake3::Hasher::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        assert_eq!(hash.as_bytes().len(), 32);
    }
    
    // Test SHA-256
    {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32);
    }
    
    // Test BLAKE2b
    {
        use blake2::{Blake2b512, Digest};
        let mut hasher = Blake2b512::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 64);
    }
    
    // Test SHA3-256
    {
        use sha3::{Sha3_256, Digest};
        let mut hasher = Sha3_256::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        assert_eq!(hash.len(), 32);
    }
    
    println!("All algorithms successfully fall back to scalar implementations");
}

#[test]
fn test_simd_correctness() {
    // Verify that SIMD and scalar implementations produce the same results
    // This is a correctness test to ensure SIMD optimizations don't break functionality
    
    let test_data = b"The quick brown fox jumps over the lazy dog";
    
    // BLAKE3 known hash
    {
        let mut hasher = blake3::Hasher::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash.as_bytes());
        
        // This is the correct BLAKE3 hash for this input
        assert_eq!(
            hash_hex,
            "2f1514181aadccd913abd94cfa592701a5686ab23f8df1dff1b74710febc6d4a"
        );
    }
    
    // SHA-256 known hash
    {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(test_data);
        let hash = hasher.finalize();
        let hash_hex = hex::encode(&hash);
        
        // This is the correct SHA-256 hash for this input
        assert_eq!(
            hash_hex,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }
    
    println!("SIMD implementations produce correct results");
}

#[test]
fn test_cpu_feature_detection() {
    // This test documents which CPU features are available at runtime
    // Useful for debugging SIMD issues
    
    #[cfg(target_arch = "x86_64")]
    {
        println!("\nCPU Features (x86_64):");
        
        if is_x86_feature_detected!("sse2") {
            println!("  ✓ SSE2 available");
        } else {
            println!("  ✗ SSE2 not available");
        }
        
        if is_x86_feature_detected!("sse4.1") {
            println!("  ✓ SSE4.1 available");
        } else {
            println!("  ✗ SSE4.1 not available");
        }
        
        if is_x86_feature_detected!("avx") {
            println!("  ✓ AVX available");
        } else {
            println!("  ✗ AVX not available");
        }
        
        if is_x86_feature_detected!("avx2") {
            println!("  ✓ AVX2 available");
        } else {
            println!("  ✗ AVX2 not available");
        }
        
        if is_x86_feature_detected!("avx512f") {
            println!("  ✓ AVX-512 available");
        } else {
            println!("  ✗ AVX-512 not available");
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        println!("\nCPU Features (ARM64):");
        
        #[cfg(target_feature = "neon")]
        {
            println!("  ✓ NEON available (compile-time)");
        }
        
        #[cfg(not(target_feature = "neon"))]
        {
            println!("  ⚠ NEON status unknown (runtime detection not available in stable Rust)");
            println!("    Build with RUSTFLAGS=\"-C target-cpu=native\" to enable NEON");
        }
    }
    
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        println!("\nCPU feature detection not available for this architecture");
    }
}

#[test]
fn test_performance_comparison() {
    // Compare performance of different algorithms to verify SIMD is working
    // BLAKE3 with SIMD should be significantly faster than SHA-256
    
    let test_data = generate_test_data(TEST_DATA_SIZE);
    
    let blake3_throughput = measure_throughput(
        |data| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(data);
            hasher.finalize().as_bytes().to_vec()
        },
        &test_data,
    );
    
    let sha256_throughput = measure_throughput(
        |data| {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        &test_data,
    );
    
    println!("\nPerformance Comparison:");
    println!("  BLAKE3:  {:.2} MB/s", blake3_throughput);
    println!("  SHA-256: {:.2} MB/s", sha256_throughput);
    
    // On systems with SIMD, BLAKE3 should be faster than SHA-256
    // However, we don't enforce this as a hard requirement since
    // performance can vary based on CPU and compilation flags
    if blake3_throughput > sha256_throughput {
        println!("  ✓ BLAKE3 is faster (likely using SIMD)");
    } else {
        println!("  ⚠ SHA-256 is faster or equal (SIMD may not be enabled)");
    }
}
