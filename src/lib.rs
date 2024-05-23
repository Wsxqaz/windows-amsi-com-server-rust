#![feature(prelude_import)]
#![feature(libstd_sys_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(rt)]
#![feature(print_internals)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(coverage_attribute)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::boxed::Box;
use std::ffi::c_void;
mod provider;

use com::class;
use com::interfaces;
use com::interfaces::IUnknown;
use com::sys::HRESULT;
use com::IID;
use provider::RustAmsiProvider;

pub const CLSID_AntimalwareProvider: IID = IID {
    data1: 0xf3d06e7c,
    data2: 0x1e45,
    data3: 0x4a26,
    data4: [0x84, 0x7e, 0xf9, 0xfc, 0xde, 0xe5, 0x9b, 0xe1],
};
#[repr(transparent)]
pub struct IAntimalwareProvider {
    inner: ::core::ptr::NonNull<IAntimalwareProviderVPtr>,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for IAntimalwareProvider {}
#[automatically_derived]
impl ::core::cmp::PartialEq for IAntimalwareProvider {
    #[inline]
    fn eq(&self, other: &IAntimalwareProvider) -> bool {
        self.inner == other.inner
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for IAntimalwareProvider {}
#[automatically_derived]
impl ::core::cmp::Eq for IAntimalwareProvider {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<::core::ptr::NonNull<IAntimalwareProviderVPtr>>;
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for IAntimalwareProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "IAntimalwareProvider",
            "inner",
            &&self.inner,
        )
    }
}
impl IAntimalwareProvider {
    #[allow(non_snake_case)]
    #[allow(clippy::from_over_into)]
    unsafe fn Scan<
        'a,
        __0: ::core::convert::Into<::com::Param<'a, *mut c_void>>,
        __1: ::core::convert::Into<::com::Param<'a, *mut u8>>,
    >(
        &self,
        stream: __0,
        result: __1,
    ) -> HRESULT {
        let mut param = stream.into();
        let stream = param.get_abi();
        let mut param = result.into();
        let result = param.get_abi();
        let interface_ptr = <Self as ::com::AbiTransferable>::get_abi(self);
        (interface_ptr.as_ref().as_ref().Scan)(interface_ptr, stream, result)
    }
    #[allow(non_snake_case)]
    #[allow(clippy::from_over_into)]
    unsafe fn CloseSession<'a, __0: ::core::convert::Into<::com::Param<'a, u64>>>(
        &self,
        session: __0,
    ) {
        let mut param = session.into();
        let session = param.get_abi();
        let interface_ptr = <Self as ::com::AbiTransferable>::get_abi(self);
        (interface_ptr.as_ref().as_ref().CloseSession)(interface_ptr, session)
    }
    #[allow(non_snake_case)]
    #[allow(clippy::from_over_into)]
    unsafe fn DisplayName<'a, __0: ::core::convert::Into<::com::Param<'a, *mut u16>>>(
        &self,
        f: __0,
    ) -> HRESULT {
        let mut param = f.into();
        let f = param.get_abi();
        let interface_ptr = <Self as ::com::AbiTransferable>::get_abi(self);
        (interface_ptr.as_ref().as_ref().DisplayName)(interface_ptr, f)
    }
}
impl ::core::ops::Deref for IAntimalwareProvider {
    type Target = <IAntimalwareProvider as ::com::Interface>::Super;
    fn deref(&self) -> &Self::Target {
        unsafe { ::core::mem::transmute(self) }
    }
}
impl Drop for IAntimalwareProvider {
    fn drop(&mut self) {
        unsafe {
            <Self as ::com::Interface>::as_iunknown(self).Release();
        }
    }
}
impl ::core::clone::Clone for IAntimalwareProvider {
    fn clone(&self) -> Self {
        unsafe {
            <Self as ::com::Interface>::as_iunknown(self).AddRef();
        }
        Self { inner: self.inner }
    }
}
#[allow(non_snake_case, missing_docs)]
#[repr(C)]
pub struct IAntimalwareProviderVTable {
    pub parent: <IUnknown as com::Interface>::VTable,
    pub Scan: unsafe extern "system" fn(
        ::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <*mut c_void as ::com::AbiTransferable>::Abi,
        <*mut u8 as ::com::AbiTransferable>::Abi,
    ) -> HRESULT,
    pub CloseSession: unsafe extern "system" fn(
        ::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <u64 as ::com::AbiTransferable>::Abi,
    ),
    pub DisplayName: unsafe extern "system" fn(
        ::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <*mut u16 as ::com::AbiTransferable>::Abi,
    ) -> HRESULT,
}
#[allow(missing_docs)]
pub type IAntimalwareProviderVPtr = ::core::ptr::NonNull<IAntimalwareProviderVTable>;
unsafe impl com::Interface for IAntimalwareProvider {
    type VTable = IAntimalwareProviderVTable;
    type Super = IUnknown;
    const IID: com::sys::IID = IID_IANTIMALWARE_PROVIDER;
}
#[allow(missing_docs)]
pub const IID_IANTIMALWARE_PROVIDER: com::sys::IID = com::sys::IID {
    data1: 0xb2cabfe3,
    data2: 0xfe04,
    data3: 0x42b1,
    data4: [0xa5, 0xdf, 0x08, 0xd4, 0x83, 0xd4, 0xd1, 0x25],
};
impl ::core::convert::From<IAntimalwareProvider> for IUnknown {
    fn from(this: IAntimalwareProvider) -> Self {
        unsafe { ::core::mem::transmute(this) }
    }
}
impl<'a> ::core::convert::From<&'a IAntimalwareProvider> for &'a IUnknown {
    fn from(this: &'a IAntimalwareProvider) -> Self {
        unsafe { ::core::mem::transmute(this) }
    }
}
#[allow(clippy::from_over_into)]
impl<'a> ::core::convert::Into<::com::Param<'a, IUnknown>> for IAntimalwareProvider {
    fn into(self) -> ::com::Param<'a, IUnknown> {
        ::com::Param::Owned(self.into())
    }
}
#[allow(clippy::from_over_into)]
impl<'a> ::core::convert::Into<::com::Param<'a, IUnknown>> for &'a IAntimalwareProvider {
    fn into(self) -> ::com::Param<'a, IUnknown> {
        ::com::Param::Borrowed(self.into())
    }
}

static mut _HMODULE: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

#[no_mangle]
unsafe extern "system" fn DllMain(
    hinstance: *mut ::core::ffi::c_void,
    fdw_reason: u32,
    reserved: *mut ::core::ffi::c_void,
) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if fdw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            _HMODULE = hinstance;
        }
    }
    1
}
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(
    class_id: *const ::com::sys::CLSID,
    iid: *const ::com::sys::IID,
    result: *mut *mut ::core::ffi::c_void,
) -> ::com::sys::HRESULT {
    println!("DllGetClassObject");
    println!("class_id: {0:x?}", class_id);
    use ::com::interfaces::IUnknown;
    if !!class_id.is_null() {
        {
            ::std::rt::begin_panic("class id passed to DllGetClassObject should never be null");
        }
    };
    if class_id == &CLSID_AntimalwareProvider {
        println!("CLSID_AntimalwareProvider");
        let class_id = unsafe { &*class_id };
        let instance = <RustAmsiProvider as ::com::production::Class>::Factory::allocate();
        instance.QueryInterface(&*iid, result)
    } else {
        ::com::sys::CLASS_E_CLASSNOTAVAILABLE
    }
}
#[no_mangle]
extern "system" fn DllRegisterServer() -> ::com::sys::HRESULT {
    ::com::production::registration::dll_register_server(&mut get_relevant_registry_keys())
}
#[no_mangle]
extern "system" fn DllUnregisterServer() -> ::com::sys::HRESULT {
    ::com::production::registration::dll_unregister_server(&mut get_relevant_registry_keys())
}
fn get_relevant_registry_keys() -> Vec<::com::production::registration::RegistryKeyInfo> {
    use ::com::production::registration::RegistryKeyInfo;
    let file_path = unsafe { ::com::production::registration::get_dll_file_path(_HMODULE) };
    <[_]>::into_vec(
        #[rustc_box]
        Box::new([
            RegistryKeyInfo::new(
                &::com::production::registration::class_key_path(CLSID_AntimalwareProvider),
                "",
                "RustAmsiProvider",
            ),
            RegistryKeyInfo::new(
                &::com::production::registration::class_inproc_key_path(CLSID_AntimalwareProvider),
                "",
                &file_path,
            ),
        ]),
    )
}
