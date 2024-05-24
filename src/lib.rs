#![feature(prelude_import)]
#![feature(print_internals)]
#![allow(non_snake_case)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use libc::wcslen;
use std::ffi::c_void;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr;
use widestring::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Antimalware::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Registry::*;
use windows::Win32::System::SystemServices::*;

#[no_mangle]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            CURRENT_MODULE = Some(dll_module);
        },
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    true
}

const CLSID: &str = "{f3d06e7c-1e45-4a26-847e-f9fcdee59be1}";
#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}

#[implement(IAntimalwareProvider)]
struct RustAmsiProvider {}

impl IAntimalwareProvider_Impl for RustAmsiProvider {
    fn Scan(&self, _stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT> {
        println!("Scan");
        Ok(AMSI_RESULT_CLEAN)
    }

    fn CloseSession(&self, _session: u64) {
        println!("CloseSession");
    }

    fn DisplayName(&self) -> Result<PWSTR> {
        println!("DisplayName");
        Ok(PWSTR(unsafe {
            U16CString::from_str_unchecked("RustAmsiProvider").as_mut_ptr()
        }))
    }
}

#[implement(IClassFactory)]
struct RustAmsiProviderFactory {}

impl IClassFactory_Impl for RustAmsiProviderFactory {
    fn CreateInstance(
        &self,
        _outer: Option<&IUnknown>,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> Result<()> {
        println!("CreateInstance");
        let provider: Pin<Box<IAntimalwareProvider>> = Box::pin(RustAmsiProvider {}.into());
        let provider: ManuallyDrop<Pin<Box<IAntimalwareProvider>>> = ManuallyDrop::new(provider);
        unsafe {
            *ppv = std::mem::transmute(provider);
        }
        println!("*ppv: {:?}", unsafe { *ppv });
        Ok(())
    }

    fn LockServer(&self, _lock: BOOL) -> Result<()> {
        println!("LockServer");
        Ok(())
    }
}

#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    println!("DllGetClassObject");

    let factory = RustAmsiProviderFactory {};
    let factory: Pin<Box<RustAmsiProviderFactory>> = Box::pin(factory);
    let factory: ManuallyDrop<Pin<Box<RustAmsiProviderFactory>>> = ManuallyDrop::new(factory);
    println!("*ppv: {:?}", unsafe { *ppv });

    return S_OK;
}
#[no_mangle]
extern "system" fn DllRegisterServer() -> HRESULT {
    let def_clsid_path = {
        let res = {
            let res = fmt::format(format_args!("Software\\Classes\\CLSID\\{0}", CLSID));
            res
        };
        res
    };
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let def_clsid_path_value = U16CString::from_str("RustAmsiProvider").unwrap();
    let def_clsid_path_value_len = unsafe { (wcslen(def_clsid_path_value.as_ptr()) + 1) * 2 };
    let resp = unsafe {
        RegSetKeyValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(def_clsid_path.as_ptr()),
            PCWSTR(ptr::null()),
            REG_SZ.0,
            Some(def_clsid_path_value.as_ptr() as *const _),
            ((wcslen(def_clsid_path_value.as_ptr()) + 1) * 2) as u32,
        )
        .to_hresult()
    };
    if resp != S_OK {
        return resp;
    }
    let def_clsid_inproc_path = {
        let res = {
            let res = fmt::format(format_args!(
                "Software\\Classes\\CLSID\\{0}\\InProcServer32",
                CLSID
            ));
            res
        };
        res
    };
    let def_clsid_inproc_path = U16CString::from_str(def_clsid_inproc_path).unwrap();
    let def_clsid_inproc_path_value =
        U16CString::from_str("C:\\Users\\Administrator\\windows_amsi_provider_rust.dll").unwrap();
    let def_clsid_inproc_path_value_len =
        unsafe { (wcslen(def_clsid_inproc_path_value.as_ptr()) + 1) * 2 };
    let resp = unsafe {
        RegSetKeyValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(def_clsid_inproc_path.as_ptr()),
            PCWSTR(ptr::null()),
            REG_SZ.0,
            Some(def_clsid_inproc_path_value.as_ptr() as *const _),
            def_clsid_inproc_path_value_len as u32,
        )
        .to_hresult()
    };
    if resp != S_OK {
        return resp;
    }
    let def_clsid_inproc_threading_path = {
        let res = {
            let res = fmt::format(format_args!(
                "Software\\Classes\\CLSID\\{0}\\InProcServer32",
                CLSID
            ));
            res
        };
        res
    };
    let def_clsid_inproc_threading_path =
        U16CString::from_str(def_clsid_inproc_threading_path).unwrap();
    let def_clsid_inproc_threading_key = U16CString::from_str("ThreadingModel").unwrap();
    let def_clsid_inproc_threading_value = U16CString::from_str("Both").unwrap();
    let resp = unsafe {
        RegSetKeyValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(def_clsid_inproc_threading_path.as_ptr()),
            PCWSTR(def_clsid_inproc_threading_key.as_ptr()),
            REG_SZ.0,
            Some(def_clsid_inproc_threading_value.as_ptr() as *const _),
            ((wcslen(def_clsid_inproc_threading_value.as_ptr()) + 1) * 2) as u32,
        )
        .to_hresult()
    };
    if resp != S_OK {
        return resp;
    }
    let def_amsi_provider_path = {
        let res = {
            let res = fmt::format(format_args!(
                "Software\\Microsoft\\AMSI\\Providers\\{0}",
                CLSID
            ));
            res
        };
        res
    };
    let def_amsi_provider_path = U16CString::from_str(def_amsi_provider_path).unwrap();
    let def_amsi_provider_path_value = U16CString::from_str("RustAmsiProvider").unwrap();
    let resp = unsafe {
        RegSetKeyValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(def_amsi_provider_path.as_ptr()),
            PCWSTR(ptr::null()),
            REG_SZ.0,
            Some(def_amsi_provider_path_value.as_ptr() as *const _),
            ((wcslen(def_amsi_provider_path_value.as_ptr()) + 1) * 2) as u32,
        )
        .to_hresult()
    };
    if resp != S_OK {
        return resp;
    }
    S_OK
}
#[no_mangle]
#[allow(dead_code)]
extern "system" fn DllUnregisterServer() -> HRESULT {
    let def_amsi_provider_path = {
        let res = {
            let res = fmt::format(format_args!(
                "Software\\Microsoft\\AMSI\\Providers\\{0}",
                CLSID
            ));
            res
        };
        res
    };
    let def_amsi_provider_path = U16CString::from_str(def_amsi_provider_path).unwrap();
    let resp = unsafe {
        RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_amsi_provider_path.as_ptr())).to_hresult()
    };
    if resp != S_OK {
        return resp;
    }
    let def_clsid_path = {
        let res = {
            let res = fmt::format(format_args!("Software\\Classes\\CLSID\\{0}", CLSID));
            res
        };
        res
    };
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let resp =
        unsafe { RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_clsid_path.as_ptr())).to_hresult() };
    if resp != S_OK {
        return resp;
    }
    S_OK
}
