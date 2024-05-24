use com::class;
use std::ffi::c_void;
use com::sys::{ S_OK, HRESULT };
use com::interfaces::iunknown::IUnknown;
use com::interfaces;


interfaces! {
    #[uuid("b2cabfe3-fe04-42b1-a5df-08d483d4d125")]
    pub unsafe interface IAntimalwareProvider: com::interfaces::IUnknown {
        fn Scan(&self, buffer: *const u8, result: *mut u32) -> HRESULT;
        fn CloseSession(&self, session: u32);
        fn DisplayName(&self, name: *mut u16) -> HRESULT;
    }
}

class! {
    pub class RustAmsiProvider: IAntimalwareProvider {}

    impl IAntimalwareProvider for RustAmsiProvider {
        fn Scan(
            &self,
            stream: *const u8,
            result: *mut u32
        ) ->  HRESULT {
            println!("Scan");
            println!("&self: {:x?}", &self as *const _);
            println!("stream: {:x?}", stream);
            println!("result: {:x?}", result);
            S_OK
        }

        fn CloseSession(&self, session: u32) {
            println!("CloseSession");
        }

        fn DisplayName(&self, f: *mut u16) -> HRESULT {
            println!("DisplayName");
            S_OK
        }
    }
}

