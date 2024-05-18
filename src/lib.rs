#![allow(non_snake_case)]
use libc::wcslen;
use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::ptr;
use widestring::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Antimalware::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Registry::*;
use windows::Win32::System::SystemServices::*;

static mut CURRENT_MODULE: Option<HMODULE> = None;

#[no_mangle]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                CURRENT_MODULE = Some(dll_module);
            }
            let _ = unsafe { DisableThreadLibraryCalls(dll_module) };
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    true
}

#[no_mangle]
extern "system" fn SetKeyStringValue(
    key: HKEY,
    sub_key: PCWSTR,
    value_name: PCWSTR,
    value: PCWSTR,
) -> HRESULT {
    let status = unsafe {
        RegSetKeyValueW(
            key,
            sub_key,
            value_name,
            REG_SZ.0,
            Some(value.0 as *const _),
            (wcslen(value.0) + 1) as u32,
        )
    };

    status.to_hresult()
}

#[implement(IAntimalwareProvider)]
struct AntimalwareProvider();

impl IAntimalwareProvider_Impl for AntimalwareProvider {
    fn Scan(&self, _stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("C:\\Users\\Administrator\\amsi_scan_log.txt")
            .unwrap();

        writeln!(file, "Hello, world!").unwrap();

        Ok(AMSI_RESULT_CLEAN)
    }
    fn CloseSession(&self, _session: u64) -> () {}
    fn DisplayName(&self) -> Result<PWSTR> {
        Ok(PWSTR(U16String::from_str("RustAmsiProvider").as_mut_ptr()))
    }
}

static mut PROVIDER: AntimalwareProvider = AntimalwareProvider();
const CLSID: &str = "{f3d06e7c-1e45-4a26-847e-f9fcdee59be1}";

#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}

#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let str_rclsid: &mut [u16] = &mut [0; 100];
    let str_riid: &mut [u16] = &mut [0; 100];

    let _ = unsafe {
        StringFromGUID2(rclsid, str_rclsid);
        StringFromGUID2(riid, str_riid);
    };

    unsafe {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("C:\\Users\\Administrator\\dll_get_class_object_log.txt")
            .unwrap();
        writeln!(log_file, "=== START DllGetClassObject ===").unwrap();
        writeln!(log_file, "rclsid: {:?}", rclsid).unwrap();
        writeln!(log_file, "riid: {:?}", riid).unwrap();
        writeln!(log_file, "ppv: {:?}", ppv).unwrap();
        writeln!(
            log_file,
            "rclsid: {:?}",
            U16CString::from_ptr_str(str_rclsid.as_ptr())
        )
        .unwrap();
        writeln!(
            log_file,
            "riid: {:?}",
            U16CString::from_ptr_str(str_riid.as_ptr())
        )
        .unwrap();
        writeln!(log_file, "=== END DllGetClassObject ===").unwrap();
    }

    {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("C:\\Users\\Administrator\\dll_get_class_object_log.txt")
            .unwrap();
        writeln!(log_file, "rclsid == &IAntimalwareProvider::IID").unwrap();
    }

    println!("rclsid == &IAntimalwareProvider::IID");
    println!("&mut PROVIDER: {:?}", unsafe { &mut PROVIDER as *mut _ } );
    unsafe { *ppv = &mut PROVIDER as *mut _ as *mut c_void};
    return S_OK;
}

#[no_mangle]
extern "system" fn DllRegisterServer() -> HRESULT {
    let def_clsid_path = format!("Software\\Classes\\CLSID\\{}", CLSID);
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let def_clsid_path_value = U16CString::from_str("RustAmsiProvider").unwrap();
    let def_clsid_path_value_len = unsafe { (wcslen(def_clsid_path_value.as_ptr()) + 1) * 2 };

    {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("C:\\Users\\Administrator\\dll_register_log.txt")
            .unwrap();
        writeln!(log_file, "CLSID path: {:?}", def_clsid_path).unwrap();
        writeln!(log_file, "CLSID path value: {:?}", def_clsid_path_value).unwrap();
        writeln!(
            log_file,
            "CLSID path value len: {:?}",
            def_clsid_path_value_len
        )
        .unwrap();
    }

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

    let def_clsid_inproc_path = format!("Software\\Classes\\CLSID\\{}\\InProcServer32", CLSID);
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

    let def_clsid_inproc_threading_path =
        format!("Software\\Classes\\CLSID\\{}\\InProcServer32", CLSID);
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

    let def_amsi_provider_path = format!("Software\\Microsoft\\AMSI\\Providers\\{}", CLSID);
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
    let def_amsi_provider_path = format!("Software\\Microsoft\\AMSI\\Providers\\{}", CLSID);
    let def_amsi_provider_path = U16CString::from_str(def_amsi_provider_path).unwrap();
    let resp = unsafe {
        RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_amsi_provider_path.as_ptr())).to_hresult()
    };
    if resp != S_OK {
        return resp;
    }

    let def_clsid_path = format!("Software\\Classes\\CLSID\\{}", CLSID);
    let def_clsid_path = U16CString::from_str(def_clsid_path).unwrap();
    let resp =
        unsafe { RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(def_clsid_path.as_ptr())).to_hresult() };
    if resp != S_OK {
        return resp;
    }

    S_OK
}
