// As of may 2023, a good documentation can be found here:
// https://surma.dev/things/rust-to-webassembly/
// and https://rustwasm.github.io/docs/book/reference/debugging.html
//
// ** /!\ ** wee_alloc has a leak and is more or less deprecated, lol_alloc
// seems to be the best option for small size.
// For now, I will use the default allocator which is
// dlmalloc: https://docs.rs/dlmalloc/latest/dlmalloc/

#![no_std]

extern crate alloc;
pub use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

mod abi;
pub(crate) mod abis;
mod allocator;

pub(crate) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
    allocator::get_parameters(arg_ptr)
}

pub(crate) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
    allocator::encode_length_prefixed(data)
}

// ****************************************************************************
// a bunch of functions to simulate the host
// ****************************************************************************
#[cfg(test)]
pub(crate) mod test {
    use super::alloc::vec::Vec;
    use super::allocator;

    // The below functions will only be compiled and available during tests,
    #[cfg(test)]
    /// Simulate arguments passed by the host to the SC
    pub fn host_write_buffer(data: &[u8]) -> u32 {
        allocator::test::host_write_buffer(data)
    }

    #[cfg(test)]
    /// Simulate reading arguments passed by the host to the SC
    pub fn host_read_buffer(arg_ptr: u32) -> Vec<u8> {
        allocator::test::host_read_buffer(arg_ptr)
    }
}
