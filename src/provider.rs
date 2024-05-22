use com::class;
use std::ffi::c_void;
use com::sys::{ S_OK, HRESULT };
use com::interfaces::iunknown::IUnknown;
use crate::IAntimalwareProvider;

class! {
    pub class RustAmsiProvider: IAntimalwareProvider {}

    impl IAntimalwareProvider for RustAmsiProvider {
        fn Scan(
            &self,
            stream: *mut c_void,
            result: *mut u8
        ) ->  HRESULT {
            println!("Scan");
            println!("stream: {:x?}", stream);
            println!("result: {:x?}", result);
            S_OK
        }

        fn CloseSession(&self, session: u64) {
            println!("CloseSession");
        }

        fn DisplayName(&self, f: *mut u16) -> HRESULT {
            println!("DisplayName");
            S_OK
        }
    }
}

