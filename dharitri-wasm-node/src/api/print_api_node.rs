use crate::VmApiImpl;
use dharitri_wasm::api::{PrintApi, PrintApiImpl};

impl PrintApi for VmApiImpl {
    type PrintApiImpl = VmApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        VmApiImpl {}
    }
}

impl PrintApiImpl for VmApiImpl {}
