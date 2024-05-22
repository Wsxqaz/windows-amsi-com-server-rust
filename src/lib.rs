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
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::ffi::c_void;
use std::boxed::Box;
mod provider {




    // IUnknown                 #[uuid("00000000-0000-0000-C000-000000000046")]
    // IAntimalwareProvider     #[uuid("b2cabfe3-fe04-42b1-a5df-08d483d4d125")]



    use com::class;
    use std::ffi::c_void;
    use com::sys::{S_OK, HRESULT};
    use com::interfaces::iunknown::IUnknown;
    use crate::IAntimalwareProvider;
    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct RustAmsiProvider {
        __0_IAntimalwareProvider: &'static <IAntimalwareProvider as
        ::com::Interface>::VTable,
        __refcnt: ::core::sync::atomic::AtomicU32,
    }
    impl RustAmsiProvider {
        #[doc = r" Allocate the class casting it to the supplied interface"]
        #[doc = r""]
        #[doc =
        r" This allocates the class on the heap and pins it. This is because COM classes"]
        #[doc =
        r" must have a stable location in memory. Once a COM class is instantiated somewhere"]
        #[doc = r" it must stay there."]
        pub fn allocate() -> ::com::production::ClassAllocation<Self> {
            let instance =
                RustAmsiProvider {
                    __0_IAntimalwareProvider: &RustAmsiProvider__IAntimalwareProvider_VTABLE,
                    __refcnt: ::core::sync::atomic::AtomicU32::new(1),
                };
            let instance = ::com::alloc::boxed::Box::pin(instance);
            ::com::production::ClassAllocation::new(instance)
        }
        #[allow(non_snake_case)]
        fn Scan(&self, stream: *mut c_void, result: *mut u8) -> HRESULT {
            { ::std::io::_print(format_args!("Scan\n")); };
            { ::std::io::_print(format_args!("stream: {0:x?}\n", stream)); };
            { ::std::io::_print(format_args!("result: {0:x?}\n", result)); };
            S_OK
        }
        #[allow(non_snake_case)]
        fn CloseSession(&self, session: u64) {
            { ::std::io::_print(format_args!("CloseSession\n")); };
        }
        #[allow(non_snake_case)]
        fn DisplayName(&self, f: *mut u16) -> HRESULT {
            { ::std::io::_print(format_args!("DisplayName\n")); };
            S_OK
        }
        pub unsafe fn AddRef(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>) -> u32 {
            ::com::refcounting::addref(&self.__refcnt)
        }
        #[inline(never)]
        pub unsafe fn QueryInterface(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
            riid: *const ::com::sys::IID, ppv: *mut *mut ::core::ffi::c_void)
            -> ::com::sys::HRESULT {
            let riid = &*riid;
            let pv: *const ::core::ffi::c_void =
                if riid == &::com::interfaces::iunknown::IID_IUNKNOWN ||
                            <IAntimalwareProvider as
                                    ::com::Interface>::is_iid_in_inheritance_chain(riid) {
                        &self.__0_IAntimalwareProvider as *const _ as
                            *const ::core::ffi::c_void
                    } else {
                       *ppv = ::core::ptr::null_mut::<::core::ffi::c_void>();
                       return ::com::sys::E_NOINTERFACE;
                   };
            *ppv = pv as *mut ::core::ffi::c_void;
            self.AddRef();
            ::com::sys::NOERROR
        }
        pub fn query_interface<T: ::com::Interface>(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>)
            -> Option<T> {
            let mut result = None;
            let hr =
                unsafe {
                    self.QueryInterface(&T::IID, &mut result as *mut _ as _)
                };
            if ::com::sys::FAILED(hr) { return None; }
            if true {
                    if !result.is_some() {
                            {
                                ::std::rt::begin_panic("Successful call to query_interface yielded a null pointer");
                            }
                        };
                };
            result
        }
    }
    unsafe impl com::production::Class for RustAmsiProvider {
        type Factory = RustAmsiProviderClassFactory;
        unsafe fn dec_ref_count(&self) -> u32 {
            ::com::refcounting::release(&self.__refcnt)
        }
        unsafe fn add_ref(&self) -> u32 {
            ::com::refcounting::addref(&self.__refcnt)
        }
    }
    #[allow(non_upper_case_globals)]
    static RustAmsiProvider__IAntimalwareProvider_VTABLE:
        <IAntimalwareProvider as ::com::Interface>::VTable =
        {
            type IAntimalwareProviderVTable =
                <IAntimalwareProvider as ::com::Interface>::VTable;
            IAntimalwareProviderVTable {
                parent: {
                    type IUknownVTable =
                        <::com::interfaces::IUnknown as ::com::Interface>::VTable;
                    unsafe extern "system" fn AddRef(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>) -> u32 {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProvider);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        munged.AddRef()
                    }
                    unsafe extern "system" fn Release(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>) -> u32 {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProvider);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        let new_ref_count =
                            ::com::refcounting::release(&munged.__refcnt);
                        if new_ref_count == 0 { munged.drop_inner(); }
                        new_ref_count
                    }
                    unsafe extern "system" fn QueryInterface(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>,
                        riid: *const ::com::sys::IID,
                        ppv: *mut *mut ::core::ffi::c_void) -> ::com::sys::HRESULT {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProvider);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        munged.QueryInterface(riid, ppv)
                    }
                    IUknownVTable { AddRef, Release, QueryInterface }
                },
                Scan: {
                    #[allow(non_snake_case)]
                    unsafe extern "system" fn Scan(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                        stream: <*mut c_void as ::com::AbiTransferable>::Abi,
                        result: <*mut u8 as ::com::AbiTransferable>::Abi)
                        -> HRESULT {
                        let this = this.as_ptr().sub(0usize);
                        let this =
                            ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(this
                                            as *mut _ as *mut RustAmsiProvider));
                        let stream =
                            <*mut c_void as ::com::AbiTransferable>::from_abi(stream);
                        let result =
                            <*mut u8 as ::com::AbiTransferable>::from_abi(result);
                        RustAmsiProvider::Scan(&this, stream, result)
                    }
                    Scan
                },
                CloseSession: {
                    #[allow(non_snake_case)]
                    unsafe extern "system" fn CloseSession(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                        session: <u64 as ::com::AbiTransferable>::Abi) {
                        let this = this.as_ptr().sub(0usize);
                        let this =
                            ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(this
                                            as *mut _ as *mut RustAmsiProvider));
                        let session =
                            <u64 as ::com::AbiTransferable>::from_abi(session);
                        RustAmsiProvider::CloseSession(&this, session)
                    }
                    CloseSession
                },
                DisplayName: {
                    #[allow(non_snake_case)]
                    unsafe extern "system" fn DisplayName(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                        f: <*mut u16 as ::com::AbiTransferable>::Abi) -> HRESULT {
                        let this = this.as_ptr().sub(0usize);
                        let this =
                            ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(this
                                            as *mut _ as *mut RustAmsiProvider));
                        let f = <*mut u16 as ::com::AbiTransferable>::from_abi(f);
                        RustAmsiProvider::DisplayName(&this, f)
                    }
                    DisplayName
                },
            }
        };
    impl<'a> ::core::convert::From<&'a RustAmsiProvider> for
        IAntimalwareProvider {
        fn from(class: &'a RustAmsiProvider) -> Self {
            unsafe {
                ::com::refcounting::addref(&class.__refcnt);
                ::core::mem::transmute(&class.__0_IAntimalwareProvider)
            }
        }
    }
    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct RustAmsiProviderClassFactory {
        __0_IClassFactory: &'static <::com::interfaces::IClassFactory as
        ::com::Interface>::VTable,
        __refcnt: ::core::sync::atomic::AtomicU32,
    }
    impl RustAmsiProviderClassFactory {
        #[doc = r" Allocate the class casting it to the supplied interface"]
        #[doc = r""]
        #[doc =
        r" This allocates the class on the heap and pins it. This is because COM classes"]
        #[doc =
        r" must have a stable location in memory. Once a COM class is instantiated somewhere"]
        #[doc = r" it must stay there."]
        pub fn allocate() -> ::com::production::ClassAllocation<Self> {
            let instance =
                RustAmsiProviderClassFactory {
                    __0_IClassFactory: &RustAmsiProviderClassFactory__IClassFactory_VTABLE,
                    __refcnt: ::core::sync::atomic::AtomicU32::new(1),
                };
            let instance = ::com::alloc::boxed::Box::pin(instance);
            ::com::production::ClassAllocation::new(instance)
        }
        #[allow(non_snake_case)]
        unsafe fn CreateInstance(&self,
            aggr:
                *mut ::core::ptr::NonNull<<::com::interfaces::IUnknown as
                ::com::Interface>::VTable>, riid: *const ::com::sys::IID,
            ppv: *mut *mut ::core::ffi::c_void) -> ::com::sys::HRESULT {
            if !!riid.is_null() {
                    {
                        ::std::rt::begin_panic("iid passed to CreateInstance was null");
                    }
                };
            if !aggr.is_null() { return ::com::sys::CLASS_E_NOAGGREGATION; }
            let instance = RustAmsiProvider::allocate();
            instance.QueryInterface(riid, ppv)
        }
        #[allow(non_snake_case)]
        unsafe fn LockServer(&self, _increment: com::sys::BOOL)
            -> com::sys::HRESULT {
            ::com::sys::S_OK
        }
        pub unsafe fn AddRef(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>) -> u32 {
            ::com::refcounting::addref(&self.__refcnt)
        }
        #[inline(never)]
        pub unsafe fn QueryInterface(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
            riid: *const ::com::sys::IID, ppv: *mut *mut ::core::ffi::c_void)
            -> ::com::sys::HRESULT {
            let riid = &*riid;
            let pv: *const ::core::ffi::c_void =
                if riid == &::com::interfaces::iunknown::IID_IUNKNOWN ||
                            <::com::interfaces::IClassFactory as
                                    ::com::Interface>::is_iid_in_inheritance_chain(riid) {
                        &self.__0_IClassFactory as *const _ as
                            *const ::core::ffi::c_void
                    } else {
                       *ppv = ::core::ptr::null_mut::<::core::ffi::c_void>();
                       return ::com::sys::E_NOINTERFACE;
                   };
            *ppv = pv as *mut ::core::ffi::c_void;
            self.AddRef();
            ::com::sys::NOERROR
        }
        pub fn query_interface<T: ::com::Interface>(self:
                &::core::pin::Pin<::com::alloc::boxed::Box<Self>>)
            -> Option<T> {
            let mut result = None;
            let hr =
                unsafe {
                    self.QueryInterface(&T::IID, &mut result as *mut _ as _)
                };
            if ::com::sys::FAILED(hr) { return None; }
            if true {
                    if !result.is_some() {
                            {
                                ::std::rt::begin_panic("Successful call to query_interface yielded a null pointer");
                            }
                        };
                };
            result
        }
    }
    unsafe impl com::production::Class for RustAmsiProviderClassFactory {
        type Factory = ();
        unsafe fn dec_ref_count(&self) -> u32 {
            ::com::refcounting::release(&self.__refcnt)
        }
        unsafe fn add_ref(&self) -> u32 {
            ::com::refcounting::addref(&self.__refcnt)
        }
    }
    #[allow(non_upper_case_globals)]
    static RustAmsiProviderClassFactory__IClassFactory_VTABLE:
        <::com::interfaces::IClassFactory as ::com::Interface>::VTable =
        {
            type IClassFactoryVTable =
                <::com::interfaces::IClassFactory as
                ::com::Interface>::VTable;
            IClassFactoryVTable {
                parent: {
                    type IUknownVTable =
                        <::com::interfaces::IUnknown as ::com::Interface>::VTable;
                    unsafe extern "system" fn AddRef(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>) -> u32 {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProviderClassFactory);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        munged.AddRef()
                    }
                    unsafe extern "system" fn Release(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>) -> u32 {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProviderClassFactory);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        let new_ref_count =
                            ::com::refcounting::release(&munged.__refcnt);
                        if new_ref_count == 0 { munged.drop_inner(); }
                        new_ref_count
                    }
                    unsafe extern "system" fn QueryInterface(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<<::com::interfaces::IUnknown
                            as ::com::Interface>::VTable>>,
                        riid: *const ::com::sys::IID,
                        ppv: *mut *mut ::core::ffi::c_void) -> ::com::sys::HRESULT {
                        let munged = this.as_ptr().sub(0usize);
                        let munged =
                            ::com::production::ClassAllocation::from_raw(munged as
                                        *mut _ as *mut RustAmsiProviderClassFactory);
                        let mut munged = ::core::mem::ManuallyDrop::new(munged);
                        munged.QueryInterface(riid, ppv)
                    }
                    IUknownVTable { AddRef, Release, QueryInterface }
                },
                CreateInstance: {
                    #[allow(non_snake_case)]
                    unsafe extern "system" fn CreateInstance(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<IClassFactoryVTable>>,
                        aggr:
                            <*mut ::core::ptr::NonNull<<::com::interfaces::IUnknown as
                            ::com::Interface>::VTable> as ::com::AbiTransferable>::Abi,
                        riid:
                            <*const ::com::sys::IID as ::com::AbiTransferable>::Abi,
                        ppv:
                            <*mut *mut ::core::ffi::c_void as
                            ::com::AbiTransferable>::Abi) -> ::com::sys::HRESULT {
                        let this = this.as_ptr().sub(0usize);
                        let this =
                            ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(this
                                            as *mut _ as *mut RustAmsiProviderClassFactory));
                        let aggr =
                            <*mut ::core::ptr::NonNull<<::com::interfaces::IUnknown as
                                    ::com::Interface>::VTable> as
                                    ::com::AbiTransferable>::from_abi(aggr);
                        let riid =
                            <*const ::com::sys::IID as
                                    ::com::AbiTransferable>::from_abi(riid);
                        let ppv =
                            <*mut *mut ::core::ffi::c_void as
                                    ::com::AbiTransferable>::from_abi(ppv);
                        RustAmsiProviderClassFactory::CreateInstance(&this, aggr,
                            riid, ppv)
                    }
                    CreateInstance
                },
                LockServer: {
                    #[allow(non_snake_case)]
                    unsafe extern "system" fn LockServer(this:
                            ::core::ptr::NonNull<::core::ptr::NonNull<IClassFactoryVTable>>,
                        _increment: <com::sys::BOOL as ::com::AbiTransferable>::Abi)
                        -> com::sys::HRESULT {
                        let this = this.as_ptr().sub(0usize);
                        let this =
                            ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(this
                                            as *mut _ as *mut RustAmsiProviderClassFactory));
                        let _increment =
                            <com::sys::BOOL as
                                    ::com::AbiTransferable>::from_abi(_increment);
                        RustAmsiProviderClassFactory::LockServer(&this, _increment)
                    }
                    LockServer
                },
            }
        };
    impl<'a> ::core::convert::From<&'a RustAmsiProviderClassFactory> for
        ::com::interfaces::IClassFactory {
        fn from(class: &'a RustAmsiProviderClassFactory) -> Self {
            unsafe {
                ::com::refcounting::addref(&class.__refcnt);
                ::core::mem::transmute(&class.__0_IClassFactory)
            }
        }
    }
}
use com::sys::HRESULT;
use com::interfaces::IUnknown;
use provider::RustAmsiProvider;
use com::IID;
use com::class;
use com::interfaces;
pub const CLSID_AntimalwareProvider: IID =
    IID {
        data1: 0xb2cabfe3,
        data2: 0xfe04,
        data3: 0x42b1,
        data4: [0xa5, 0xdf, 0x08, 0xd4, 0x83, 0xd4, 0xd1, 0x25],
    };
#[repr(transparent)]
pub struct IAntimalwareProvider {
    inner: ::core::ptr::NonNull<IAntimalwareProviderVPtr>,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for IAntimalwareProvider { }
#[automatically_derived]
impl ::core::cmp::PartialEq for IAntimalwareProvider {
    #[inline]
    fn eq(&self, other: &IAntimalwareProvider) -> bool {
        self.inner == other.inner
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for IAntimalwareProvider { }
#[automatically_derived]
impl ::core::cmp::Eq for IAntimalwareProvider {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _:
                ::core::cmp::AssertParamIsEq<::core::ptr::NonNull<IAntimalwareProviderVPtr>>;
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for IAntimalwareProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f,
            "IAntimalwareProvider", "inner", &&self.inner)
    }
}
impl IAntimalwareProvider {
    #[allow(non_snake_case)]
    #[allow(clippy :: from_over_into)]
    unsafe fn Scan<'a,
        __0: ::core::convert::Into<::com::Param<'a, *mut c_void>>,
        __1: ::core::convert::Into<::com::Param<'a,
        *mut u8>>>(&self, stream: __0, result: __1) -> HRESULT {
        let mut param = stream.into();
        let stream = param.get_abi();
        let mut param = result.into();
        let result = param.get_abi();
        let interface_ptr = <Self as ::com::AbiTransferable>::get_abi(self);
        (interface_ptr.as_ref().as_ref().Scan)(interface_ptr, stream, result)
    }
    #[allow(non_snake_case)]
    #[allow(clippy :: from_over_into)]
    unsafe fn CloseSession<'a,
        __0: ::core::convert::Into<::com::Param<'a,
        u64>>>(&self, session: __0) {
        let mut param = session.into();
        let session = param.get_abi();
        let interface_ptr = <Self as ::com::AbiTransferable>::get_abi(self);
        (interface_ptr.as_ref().as_ref().CloseSession)(interface_ptr, session)
    }
    #[allow(non_snake_case)]
    #[allow(clippy :: from_over_into)]
    unsafe fn DisplayName<'a,
        __0: ::core::convert::Into<::com::Param<'a, *mut u16>>>(&self, f: __0)
        -> HRESULT {
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
        unsafe { <Self as ::com::Interface>::as_iunknown(self).Release(); }
    }
}
impl ::core::clone::Clone for IAntimalwareProvider {
    fn clone(&self) -> Self {
        unsafe { <Self as ::com::Interface>::as_iunknown(self).AddRef(); }
        Self { inner: self.inner }
    }
}
#[allow(non_snake_case, missing_docs)]
#[repr(C)]
pub struct IAntimalwareProviderVTable {
    pub parent: <IUnknown as com::Interface>::VTable,
    pub Scan: unsafe extern "system" fn(::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <*mut c_void as ::com::AbiTransferable>::Abi,
        <*mut u8 as ::com::AbiTransferable>::Abi) -> HRESULT,
    pub CloseSession: unsafe extern "system" fn(::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <u64 as ::com::AbiTransferable>::Abi),
    pub DisplayName: unsafe extern "system" fn(::core::ptr::NonNull<IAntimalwareProviderVPtr>,
        <*mut u16 as ::com::AbiTransferable>::Abi) -> HRESULT,
}
#[allow(missing_docs)]
pub type IAntimalwareProviderVPtr =
    ::core::ptr::NonNull<IAntimalwareProviderVTable>;
unsafe impl com::Interface for IAntimalwareProvider {
    type VTable = IAntimalwareProviderVTable;
    type Super = IUnknown;
    const IID: com::sys::IID = IID_IANTIMALWARE_PROVIDER;
}
#[allow(missing_docs)]
pub const IID_IANTIMALWARE_PROVIDER: com::sys::IID =
    com::sys::IID {
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
#[allow(clippy :: from_over_into)]
impl<'a> ::core::convert::Into<::com::Param<'a, IUnknown>> for
    IAntimalwareProvider {
    fn into(self) -> ::com::Param<'a, IUnknown> {
        ::com::Param::Owned(self.into())
    }
}
#[allow(clippy :: from_over_into)]
impl<'a> ::core::convert::Into<::com::Param<'a, IUnknown>> for
    &'a IAntimalwareProvider {
    fn into(self) -> ::com::Param<'a, IUnknown> {
        ::com::Param::Borrowed(self.into())
    }
}
static mut _HMODULE: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
#[no_mangle]
unsafe extern "system" fn DllMain(hinstance: *mut ::core::ffi::c_void,
    fdw_reason: u32, reserved: *mut ::core::ffi::c_void) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if fdw_reason == DLL_PROCESS_ATTACH { unsafe { _HMODULE = hinstance; } }
    1
}
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(class_id:
        *const ::com::sys::CLSID, iid: *const ::com::sys::IID,
    result: *mut *mut ::core::ffi::c_void) -> ::com::sys::HRESULT {
    println!("DllGetClassObject");
    println!("class_id: {0:x?}", class_id);
    use ::com::interfaces::IUnknown;
    if !!class_id.is_null() {
            {
                ::std::rt::begin_panic("class id passed to DllGetClassObject should never be null");
            }
        };
    let class_id = unsafe { &*class_id };
            let instance =
                <RustAmsiProvider as
                        ::com::production::Class>::Factory::allocate();
            instance.QueryInterface(&*iid, result)
    // if class_id == &CLSID_AntimalwareProvider {
    //     println!("CLSID_AntimalwareProvider");
    //     } else { ::com::sys::CLASS_E_CLASSNOTAVAILABLE }
}
#[no_mangle]
extern "system" fn DllRegisterServer() -> ::com::sys::HRESULT {
    ::com::production::registration::dll_register_server(&mut get_relevant_registry_keys())
}
#[no_mangle]
extern "system" fn DllUnregisterServer() -> ::com::sys::HRESULT {
    ::com::production::registration::dll_unregister_server(&mut get_relevant_registry_keys())
}
fn get_relevant_registry_keys()
    -> Vec<::com::production::registration::RegistryKeyInfo> {
    use ::com::production::registration::RegistryKeyInfo;
    let file_path =
        unsafe {
            ::com::production::registration::get_dll_file_path(_HMODULE)
        };
    <[_]>::into_vec(#[rustc_box] Box::new([RegistryKeyInfo::new(&::com::production::registration::class_key_path(CLSID_AntimalwareProvider),
                        "", "RustAmsiProvider"),
                    RegistryKeyInfo::new(&::com::production::registration::class_inproc_key_path(CLSID_AntimalwareProvider),
                        "", &file_path)]))
}

