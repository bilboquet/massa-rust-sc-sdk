use alloc::string::ToString;
use massa_proto::massa::abi::v1::{
    NativeAmount, NativeAmountFromStringRequest,
    NativeAmountFromStringResponse, NativeAmountToStringRequest,
    NativeAmountToStringResponse,
};
use prost::Message;

use crate::{
    alloc::string::String,
    allocator::{get_parameters, EncodeLengthPrefixed},
    massa_abi,
};

// ****************************************************************************
// Function from the abi related to NativeAmount

massa_abi!(abi_check_native_amount); //
massa_abi!(abi_add_native_amounts); // (amount1, amount2) -> Result<amount>
massa_abi!(abi_sub_native_amounts); // (amount2, amount2) -> Result<amount>
massa_abi!(abi_mul_native_amount); // (amount, uint64) -> Result<amount>
massa_abi!(abi_div_rem_native_amount); // (amount, uint64) -> Result<(amount, amount)> note: the second returned amount
                                       // is the remainder of the division
massa_abi!(abi_div_rem_native_amounts); // (amount, amount) -> Result<(uint64, amount)> note: the second returned amount
                                        // is the remainder of the division
massa_abi!(abi_native_amount_to_string); //
massa_abi!(abi_native_amount_from_string); //
massa_abi!(abi_native_amount_to_bytes); //
massa_abi!(abi_native_amount_from_bytes); //

// ****************************************************************************

// Interface between the sdk and the SC i.e. seen by the user
// Wrapped function to "hide" unsafe and manage serialize/deserialize of the
// parameters

pub(crate) fn native_amount_to_string(
    amount_to_convert: NativeAmount,
) -> Result<String, String> {
    {
        // serialize the arguments with protobuf
        let arg_ptr = NativeAmountToStringRequest {
            amount_to_convert: Some(amount_to_convert),
        }
        .encode_length_prefixed();

        // call the function from the abi
        let resp_ptr = unsafe { abi_native_amount_to_string(arg_ptr) };

        Ok(NativeAmountToStringResponse::decode(
            get_parameters(resp_ptr).as_slice(),
        )
        .map_err(|_| "Error decoding NativeAmountToStringResponse".to_string())?
        .converted_amount)
    }
}

pub(crate) fn native_amount_from_string(
    amount_to_convert: String,
) -> Result<NativeAmount, String> {
    // serialize the arguments with protobuf
    let arg_ptr = NativeAmountFromStringRequest { amount_to_convert }
        .encode_length_prefixed();

    // call the function from the abi
    let resp_ptr = unsafe { abi_native_amount_from_string(arg_ptr) };

    // deserialize the returned value with protobuf
    let resp = NativeAmountFromStringResponse::decode(
        get_parameters(resp_ptr).as_slice(),
    )
    .map_err(|_| "Error decoding NativeAmountFromStringResponse".to_string())?;

    Ok(resp
        .converted_amount
        .ok_or("Error converting string to NativeAmount".to_string())?)
}

#[derive(Clone)]
pub struct Amount(NativeAmount);

impl From<NativeAmount> for Amount {
    fn from(value: NativeAmount) -> Self {
        Amount(value)
    }
}

impl Into<NativeAmount> for Amount {
    fn into(self) -> NativeAmount {
        self.0
    }
}

impl TryInto<String> for Amount {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        native_amount_to_string(self.0.clone())
    }
}

impl TryFrom<String> for Amount {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(native_amount_from_string(value)?.into())
    }
}
