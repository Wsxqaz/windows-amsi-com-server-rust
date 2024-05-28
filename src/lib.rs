#![allow(non_snake_case)]
#[macro_use]
extern crate std;
use libc::wcslen;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr;
use widestring::*;
use windows::core::{implement, IUnknown, IUnknownImpl, Result, GUID, HRESULT, PCWSTR, PWSTR};
use windows::Win32::Foundation::{
    BOOL, E_FAIL, HMODULE, MAX_PATH, S_OK, ERROR_INSUFFICIENT_BUFFER,
};
use windows::Win32::System::Antimalware::{
    IAmsiStream, IAntimalwareProvider, IAntimalwareProvider_Impl, AMSI_RESULT, AMSI_RESULT_CLEAN, AMSI_ATTRIBUTE_APP_NAME, AMSI_ATTRIBUTE,
    AMSI_ATTRIBUTE_CONTENT_NAME, AMSI_ATTRIBUTE_CONTENT_SIZE, AMSI_ATTRIBUTE_CONTENT_ADDRESS,
};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::{
    RegDeleteTreeW, RegSetKeyValueW, HKEY_LOCAL_MACHINE, REG_SZ,
};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

const CLSID: &str = "{f3d06e7c-1e45-4a26-847e-f9fcdee59be1}";
static mut DLL_MODULE: Option<HMODULE> = None;

#[no_mangle]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let _ = env_logger::try_init();
            unsafe {
                DLL_MODULE = Some(dll_module);
            }
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    true
}

#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}

#[implement(IAntimalwareProvider)]
struct RustAmsiProvider {}
impl IAntimalwareProvider_Impl for RustAmsiProvider {
    fn Scan(&self, stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT> {
        log::info!("IAntimalwareProvider_Impl::Scan");

        let stream: &IAmsiStream = stream.as_ref().unwrap();

        let mut app_name: [u8; 1024] = [0u8; 1024];
        let mut app_name_len = 0;
        let resp = unsafe { &stream.GetAttribute(AMSI_ATTRIBUTE_APP_NAME, &mut app_name, &mut app_name_len) };
        let app_name = unsafe { U16CString::from_ptr_truncate(app_name.as_mut_ptr() as *mut u16, app_name_len as usize) };
        log::info!("App Name: {:?}", app_name);

        let mut content_name: [u8; 1024] = [0u8; 1024];
        let mut content_name_len = 0;
        let resp = unsafe { &stream.GetAttribute(AMSI_ATTRIBUTE_CONTENT_NAME, &mut content_name , &mut content_name_len) };
        let content_name = unsafe { U16CString::from_ptr_truncate(content_name.as_mut_ptr() as *mut u16, content_name_len as usize) };
        log::info!("Content Name: {:?}", content_name);

        let mut content_size: [u8; 8] = [0u8; 8];
        let mut content_len = 0;
        let resp = unsafe { &stream.GetAttribute(AMSI_ATTRIBUTE_CONTENT_SIZE, &mut content_size, &mut content_len) };
        let content_size = unsafe { usize::from_ne_bytes(content_size) };
        log::info!("Content Size: {:?}", content_size);

        let mut content_address: [u8; 8] = [0u8; 8];
        let mut content_address_len = 0;
        let resp = unsafe { &stream.GetAttribute(AMSI_ATTRIBUTE_CONTENT_ADDRESS, &mut content_address, &mut content_address_len) };
        let content_address = unsafe { u64::from_ne_bytes(content_address) as *const u16 };
        log::info!("Content Address: {:?}", content_address);

        let content_str = unsafe { U16CString::from_ptr_truncate(content_address, content_size) };
        log::info!("Content: {:?}", content_str);

        Ok(AMSI_RESULT_CLEAN)
    }
    fn CloseSession(&self, _session: u64) {
        log::info!("IAntimalwareProvider_Impl::CloseSession");
    }
    fn DisplayName(&self) -> Result<PWSTR> {
        log::info!("IAntimalwareProvider_Impl::DisplayName");
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
        log::info!("IClassFactory_Impl::CreateInstance");
        let provider: ManuallyDrop<Pin<Box<RustAmsiProvider_Impl>>> =
            ManuallyDrop::new(Box::pin(RustAmsiProvider_Impl::new(RustAmsiProvider {})));
        let _ = unsafe { provider.QueryInterface(riid, ppv) };
        Ok(())
    }
    fn LockServer(&self, _lock: BOOL) -> Result<()> {
        log::info!("IClassFactory_Impl::LockServer");
        Ok(())
    }
}

#[no_mangle]
extern "system" fn DllGetClassObject(
    _rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    log::info!("DllGetClassObject");
    let factory: ManuallyDrop<Pin<Box<RustAmsiProviderFactory_Impl>>> =
        ManuallyDrop::new(Box::pin(RustAmsiProviderFactory_Impl::new(
            RustAmsiProviderFactory {},
        )));
    let _ = unsafe { factory.QueryInterface(riid, ppv) };
    return S_OK;
}

#[no_mangle]
extern "system" fn DllRegisterServer() -> HRESULT {
    let def_clsid_path = format!("Software\\Classes\\CLSID\\{0}", CLSID);
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let def_clsid_path_value = U16CString::from_str("RustAmsiProvider").unwrap();
    let def_clsid_path_value_len =
        unsafe { ((wcslen(def_clsid_path_value.as_ptr()) + 1) * 2) as u32 };
    let resp = unsafe {
        RegSetKeyValueW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(def_clsid_path.as_ptr()),
            PCWSTR(ptr::null()),
            REG_SZ.0,
            Some(def_clsid_path_value.as_ptr() as *const _),
            def_clsid_path_value_len as u32,
        )
        .to_hresult()
    };
    if resp != S_OK {
        return resp;
    }

    let def_clsid_inproc_path = format!("Software\\Classes\\CLSID\\{0}\\InProcServer32", CLSID);
    let def_clsid_inproc_path = U16CString::from_str(def_clsid_inproc_path).unwrap();
    let def_clsid_inproc_path_value = {
        if let Some(dll_module) = unsafe { DLL_MODULE } {
            let mut path = [0u16; MAX_PATH as usize];
            let len = unsafe { GetModuleFileNameW(dll_module, &mut path) };
            if len != 0 {
                let path = unsafe { U16CString::from_raw(path.as_mut_ptr()) };
                path
            } else {
                return E_FAIL;
            }
        } else {
            return E_FAIL;
        }
    };

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

    let def_clsid_inproc_threading_path =
        format!("Software\\Classes\\CLSID\\{0}\\InProcServer32", CLSID);
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

    let def_amsi_provider_path = format!("Software\\Microsoft\\AMSI\\Providers\\{0}", CLSID);
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
    let def_amsi_provider_path = format!("Software\\Microsoft\\AMSI\\Providers\\{0}", CLSID);
    let def_amsi_provider_path = U16CString::from_str(def_amsi_provider_path).unwrap();
    let resp = unsafe {
        RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_amsi_provider_path.as_ptr())).to_hresult()
    };
    if resp != S_OK {
        return resp;
    }

    let def_clsid_path = format!("Software\\Classes\\CLSID\\{0}", CLSID);
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let resp =
        unsafe { RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_clsid_path.as_ptr())).to_hresult() };
    if resp != S_OK {
        return resp;
    }

    S_OK
}
