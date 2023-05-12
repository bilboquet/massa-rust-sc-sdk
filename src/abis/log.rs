use crate::alloc::string::String;

use cfg_if::cfg_if;
use prost::Message;

// Include the `abi` module, which is generated from the proto files.
use crate::abi::proto::massa::abi::v1::LogRequest;

use crate::allocator::encode_length_prefixed;

// ****************************************************************************
// Function from the abi used by the SC

// Interface between the sdk and the abi

// specify the "namespace" of the function being imported i.e. "massa"
#[link(wasm_import_module = "massa")]
// specify the function name as it is in the abi
// CHECK: extern "C" implies no_mangle
extern "C" {
    // may be use to "rename" a function
    // #[link_name = "actual_symbol_name"]
    fn abi_log(arg: u32) -> u32;
}

// ****************************************************************************

// Interface between the sdk and the SC i.e. seen by the user
// Wrapped function to "hide" unsafe and manage serialize/deserialize of the
// parameters
fn impl_log(arg: String) {
    // serialize the arguments with protobuf
    let req = LogRequest { message: arg };
    let req_bytes = req.encode_to_vec();

    let arg_ptr: u32 = encode_length_prefixed(req_bytes);

    // call the function from the abi
    unsafe { abi_log(arg_ptr) };
}

// ****************************************************************************
// mocked version of the abi so one can dev and write tests without the need
// to call the host
cfg_if! {
    if #[cfg(test)] {
        extern crate std;
        use std::println;

        // Should we leave it up to the user to implement the mock?
        // Should we mock at the abi_level?
        // Can mockall do the job?
        fn mock_log(arg: String)  {
            println!("{}", arg);
        }
    }
}

pub fn log(arg: String) {
    cfg_if! {
        if #[cfg(test)]    {
            mock_log(arg)
        }
         else {
            impl_log(arg)
        }
    }
}