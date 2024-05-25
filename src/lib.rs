#![feature(prelude_import)]
#![feature(print_internals)]
#![allow(non_snake_case)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
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
        DLL_PROCESS_ATTACH => {},
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

#[repr(C)]
struct RustAmsiProvider_Impl {
    identity: *const ::windows_core::IInspectable_Vtbl,
    vtables: (*const <IAntimalwareProvider as ::windows_core::Interface>::Vtable,),
    this: RustAmsiProvider,
    count: ::windows_core::imp::WeakRefCount,
}
impl RustAmsiProvider_Impl {
    const VTABLES: (<IAntimalwareProvider as ::windows_core::Interface>::Vtable,) = (
        <IAntimalwareProvider as ::windows_core::Interface>::Vtable::new::<
            Self,
            RustAmsiProvider,
            -1,
        >(),
    );
    const IDENTITY: ::windows_core::IInspectable_Vtbl =
        ::windows_core::IInspectable_Vtbl::new::<Self, IAntimalwareProvider, 0>();
    fn new(this: RustAmsiProvider) -> Self {
        Self {
            identity: &Self::IDENTITY,
            vtables: (&Self::VTABLES.0,),
            this,
            count: ::windows_core::imp::WeakRefCount::new(),
        }
    }
}
impl ::windows_core::IUnknownImpl for RustAmsiProvider_Impl {
    type Impl = RustAmsiProvider;
    fn get_impl(&self) -> &Self::Impl {
        &self.this
    }
    unsafe fn QueryInterface(
        &self,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        println!("RustAmsiProvider_Impl::QueryInterface");
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        let iid = &*iid;
        *interface = if iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID
            || iid == &<::windows_core::IInspectable as ::windows_core::Interface>::IID
            || iid == &<::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID
        {
            println!("RustAmsiProvider_Impl::QueryInterface IUnknown");
            &self.identity as *const _ as *mut _
        } else if <IAntimalwareProvider as ::windows_core::Interface>::Vtable::matches(iid) {
            println!("RustAmsiProvider_Impl::QueryInterface IAntimalwareProvider");
            &self.vtables.0 as *const _ as *mut _
        } else {
            println!("RustAmsiProvider_Impl::QueryInterface null");
            ::core::ptr::null_mut()
        };
        if !(*interface).is_null() {
            self.count.add_ref();
            return ::windows_core::HRESULT(0);
        }
        *interface = self.count.query(iid, &self.identity as *const _ as *mut _);
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            ::windows_core::HRESULT(0)
        }
    }
    fn AddRef(&self) -> u32 {
        self.count.add_ref()
    }
    unsafe fn Release(&self) -> u32 {
        let remaining = self.count.release();
        if remaining == 0 {
            _ = ::std::boxed::Box::from_raw(self as *const Self as *mut Self);
        }
        remaining
    }
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
        if value.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *value = 0;
        ::windows_core::HRESULT(0)
    }
}
impl RustAmsiProvider {
    #[doc = r" Try casting as the provided interface"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" This function can only be safely called if `self` has been heap allocated and pinned using"]
    #[doc = r" the mechanisms provided by `implement` macro."]
    unsafe fn cast<I: ::windows_core::Interface>(&self) -> ::windows_core::Result<I> {
        let boxed = (self as *const _ as *const *mut ::core::ffi::c_void).sub(1 + 1)
            as *mut RustAmsiProvider_Impl;
        let mut result = ::std::ptr::null_mut();
        _ = <RustAmsiProvider_Impl as ::windows_core::IUnknownImpl>::QueryInterface(
            &*boxed,
            &I::IID,
            &mut result,
        );
        ::windows_core::Type::from_abi(result)
    }
}
impl ::core::convert::From<RustAmsiProvider> for ::windows_core::IUnknown {
    fn from(this: RustAmsiProvider) -> Self {
        let this = RustAmsiProvider_Impl::new(this);
        let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::core::mem::transmute(&boxed.identity) }
    }
}
impl ::core::convert::From<RustAmsiProvider> for ::windows_core::IInspectable {
    fn from(this: RustAmsiProvider) -> Self {
        let this = RustAmsiProvider_Impl::new(this);
        let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::core::mem::transmute(&boxed.identity) }
    }
}
impl ::core::convert::From<RustAmsiProvider> for IAntimalwareProvider {
    fn from(this: RustAmsiProvider) -> Self {
        let this = RustAmsiProvider_Impl::new(this);
        let mut this = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        let vtable_ptr = &this.vtables.0;
        unsafe { ::core::mem::transmute(vtable_ptr) }
    }
}
impl ::windows_core::AsImpl<RustAmsiProvider> for IAntimalwareProvider {
    unsafe fn as_impl(&self) -> &RustAmsiProvider {
        let this = ::windows_core::Interface::as_raw(self);
        let this = (this as *mut *mut ::core::ffi::c_void).sub(1 + 0) as *mut RustAmsiProvider_Impl;
        &(*this).this
    }
}
struct RustAmsiProvider {}

impl IAntimalwareProvider_Impl for RustAmsiProvider {
    fn Scan(&self, _stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT> {
        {
            ::std::io::_print(format_args!("Scan\n"));
        };
        Ok(AMSI_RESULT_CLEAN)
    }
    fn CloseSession(&self, _session: u64) {
        {
            ::std::io::_print(format_args!("CloseSession\n"));
        };
    }
    fn DisplayName(&self) -> Result<PWSTR> {
        {
            ::std::io::_print(format_args!("DisplayName\n"));
        };
        Ok(PWSTR(unsafe {
            U16CString::from_str_unchecked("RustAmsiProvider").as_mut_ptr()
        }))
    }
}
#[repr(C)]
struct RustAmsiProviderFactory_Impl {
    identity: *const ::windows_core::IInspectable_Vtbl,
    vtables: (*const <IClassFactory as ::windows_core::Interface>::Vtable,),
    this: RustAmsiProviderFactory,
    count: ::windows_core::imp::WeakRefCount,
}
impl RustAmsiProviderFactory_Impl {
    const VTABLES: (<IClassFactory as ::windows_core::Interface>::Vtable,) =
        (<IClassFactory as ::windows_core::Interface>::Vtable::new::<
            Self,
            RustAmsiProviderFactory,
            -1,
        >(),);
    const IDENTITY: ::windows_core::IInspectable_Vtbl =
        ::windows_core::IInspectable_Vtbl::new::<Self, IClassFactory, 0>();
    fn new(this: RustAmsiProviderFactory) -> Self {
        Self {
            identity: &Self::IDENTITY,
            vtables: (&Self::VTABLES.0,),
            this,
            count: ::windows_core::imp::WeakRefCount::new(),
        }
    }
}
impl ::windows_core::IUnknownImpl for RustAmsiProviderFactory_Impl {
    type Impl = RustAmsiProviderFactory;
    fn get_impl(&self) -> &Self::Impl {
        &self.this
    }
    unsafe fn QueryInterface(
        &self,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        println!("RustAmsiProviderFactory_Impl::QueryInterface");
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        let iid = &*iid;
        *interface = if iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID
            || iid == &<::windows_core::IInspectable as ::windows_core::Interface>::IID
            || iid == &<::windows_core::imp::IAgileObject as ::windows_core::Interface>::IID
        {
            println!("RustAmsiProviderFactory_Impl::QueryInterface IUnknown");
            &self.identity as *const _ as *mut _
        } else if <IClassFactory as ::windows_core::Interface>::Vtable::matches(iid) {
            println!("RustAmsiProviderFactory_Impl::QueryInterface IClassFactory");
            &self.vtables.0 as *const _ as *mut _
        } else {
            println!("RustAmsiProviderFactory_Impl::QueryInterface null");
            ::core::ptr::null_mut()
        };
        if !(*interface).is_null() {
            self.count.add_ref();
            return ::windows_core::HRESULT(0);
        }
        *interface = self.count.query(iid, &self.identity as *const _ as *mut _);
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            ::windows_core::HRESULT(0)
        }
    }
    fn AddRef(&self) -> u32 {
        self.count.add_ref()
    }
    unsafe fn Release(&self) -> u32 {
        let remaining = self.count.release();
        if remaining == 0 {
            _ = ::std::boxed::Box::from_raw(self as *const Self as *mut Self);
        }
        remaining
    }
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
        if value.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *value = 0;
        ::windows_core::HRESULT(0)
    }
}
impl RustAmsiProviderFactory {
    #[doc = r" Try casting as the provided interface"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" This function can only be safely called if `self` has been heap allocated and pinned using"]
    #[doc = r" the mechanisms provided by `implement` macro."]
    unsafe fn cast<I: ::windows_core::Interface>(&self) -> ::windows_core::Result<I> {
        let boxed = (self as *const _ as *const *mut ::core::ffi::c_void).sub(1 + 1)
            as *mut RustAmsiProviderFactory_Impl;
        let mut result = ::std::ptr::null_mut();
        _ = <RustAmsiProviderFactory_Impl as ::windows_core::IUnknownImpl>::QueryInterface(
            &*boxed,
            &I::IID,
            &mut result,
        );
        ::windows_core::Type::from_abi(result)
    }
}
impl ::core::convert::From<RustAmsiProviderFactory> for ::windows_core::IUnknown {
    fn from(this: RustAmsiProviderFactory) -> Self {
        let this = RustAmsiProviderFactory_Impl::new(this);
        let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::core::mem::transmute(&boxed.identity) }
    }
}
impl ::core::convert::From<RustAmsiProviderFactory> for ::windows_core::IInspectable {
    fn from(this: RustAmsiProviderFactory) -> Self {
        let this = RustAmsiProviderFactory_Impl::new(this);
        let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::core::mem::transmute(&boxed.identity) }
    }
}
impl ::core::convert::From<RustAmsiProviderFactory> for IClassFactory {
    fn from(this: RustAmsiProviderFactory) -> Self {
        let this = RustAmsiProviderFactory_Impl::new(this);
        let mut this = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        let vtable_ptr = &this.vtables.0;
        unsafe { ::core::mem::transmute(vtable_ptr) }
    }
}
impl ::windows_core::AsImpl<RustAmsiProviderFactory> for IClassFactory {
    unsafe fn as_impl(&self) -> &RustAmsiProviderFactory {
        let this = ::windows_core::Interface::as_raw(self);
        let this =
            (this as *mut *mut ::core::ffi::c_void).sub(1 + 0) as *mut RustAmsiProviderFactory_Impl;
        &(*this).this
    }
}
struct RustAmsiProviderFactory {}
impl IClassFactory_Impl for RustAmsiProviderFactory {
    fn CreateInstance(
        &self,
        _outer: Option<&IUnknown>,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> Result<()> {
        {
            ::std::io::_print(format_args!("CreateInstance\n"));
        };
        let provider: ManuallyDrop<Pin<Box<RustAmsiProvider_Impl>>> = ManuallyDrop::new(Box::pin(RustAmsiProvider_Impl::new(RustAmsiProvider {})));
        unsafe {
            provider.QueryInterface(riid, ppv);
        }
        {
            ::std::io::_print(format_args!("CreateInstance *ppv: {0:?}\n", unsafe { *ppv }));
        };
        Ok(())
    }
    fn LockServer(&self, _lock: BOOL) -> Result<()> {
        {
            ::std::io::_print(format_args!("LockServer\n"));
        };
        Ok(())
    }
}
#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    {
        ::std::io::_print(format_args!("DllGetClassObject\n"));
    };
    let factory: ManuallyDrop<Pin<Box<RustAmsiProviderFactory_Impl>>> = ManuallyDrop::new(Box::pin(RustAmsiProviderFactory_Impl::new(RustAmsiProviderFactory {})));
    unsafe {
        factory.QueryInterface(riid, ppv);
    }
    {
        ::std::io::_print(format_args!("DllGetClassObject *ppv: {0:?}\n", unsafe { *ppv }));
    };
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
