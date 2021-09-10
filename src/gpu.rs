//! A Rust wrapper around CUDA-PTX implementation of Sloth suggested in
//! https://eprint.iacr.org/2015/366, extended for a proof-of-replication,
//! and instantiated for 2**256-189 modulus.

use std::error::Error;
use std::fmt;

#[cfg(test)]
mod tests;

/// Data bigger than the prime, this is not supported
#[derive(Debug, Copy, Clone)]
pub struct CudaError;

impl fmt::Display for CudaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Some CUDA Error occurred. Aborting...")
    }
}

impl Error for CudaError {}

// importing the functions from .c files
extern "C" {
    fn batch_encode(inout: *mut u8, len: usize, iv_: *const u8, layers: usize) -> bool;
    fn detect_cuda() -> bool;
    fn test_1x1_cuda(inout: *mut u8, len: usize, iv_: *const u8, layers: usize) -> ();
}

pub fn check_cuda() -> bool {
    unsafe {
        return detect_cuda();
    }
}

/// Sequentially encodes a 4096 byte piece s.t. a minimum amount of
/// wall clock time elapses
pub fn gpu_encode(piece: &mut Vec<u8>, iv: Vec<u8>, layers: usize) -> Result<(), CudaError> {
    unsafe {
        if batch_encode(piece.as_mut_ptr(), piece.len(), iv.as_ptr(), layers) {
            return Err(CudaError);
        }
    };
    Ok(())
}

pub fn gpu_test_single_piece(piece: &mut Vec<u8>, iv: Vec<u8>, layers: usize) -> () {
    unsafe { test_1x1_cuda(piece.as_mut_ptr(), piece.len(), iv.as_ptr(), layers) }
}