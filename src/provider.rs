
use crate::IAntimalwareProvider;
use com::class;
use com::interfaces::iunknown::IUnknown;
use com::sys::{HRESULT, S_OK};
use std::ffi::c_void;
#[repr(C)]
#[allow(non_snake_case)]
pub struct RustAmsiProvider {
    __0_IAntimalwareProvider: &'static <IAntimalwareProvider as ::com::Interface>::VTable,
    __refcnt: ::core::sync::atomic::AtomicU32,
}
impl RustAmsiProvider {
    #[doc = r" Allocate the class casting it to the supplied interface"]
    #[doc = r""]
    #[doc = r" This allocates the class on the heap and pins it. This is because COM classes"]
    #[doc = r" must have a stable location in memory. Once a COM class is instantiated somewhere"]
    #[doc = r" it must stay there."]
    pub fn allocate() -> ::com::production::ClassAllocation<Self> {
        let instance = RustAmsiProvider {
            __0_IAntimalwareProvider: &RustAmsiProvider__IAntimalwareProvider_VTABLE,
            __refcnt: ::core::sync::atomic::AtomicU32::new(1),
        };
        let instance = ::com::alloc::boxed::Box::pin(instance);
        ::com::production::ClassAllocation::new(instance)
    }
    #[allow(non_snake_case)]
    fn Scan(&self, stream: *mut c_void, result: *mut u8) -> HRESULT {
        {
            ::std::io::_print(format_args!("Scan\n"));
        };
        {
            ::std::io::_print(format_args!("stream: {0:x?}\n", stream));
        };
        {
            ::std::io::_print(format_args!("result: {0:x?}\n", result));
        };
        S_OK
    }
    #[allow(non_snake_case)]
    fn CloseSession(&self, session: u64) {
        {
            ::std::io::_print(format_args!("CloseSession\n"));
        };
    }
    #[allow(non_snake_case)]
    fn DisplayName(&self, f: *mut u16) -> HRESULT {
        {
            ::std::io::_print(format_args!("DisplayName\n"));
        };
        S_OK
    }
    pub unsafe fn AddRef(self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>) -> u32 {
        ::com::refcounting::addref(&self.__refcnt)
    }
    #[inline(never)]
    pub unsafe fn QueryInterface(
        self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
        riid: *const ::com::sys::IID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::com::sys::HRESULT {
        let riid = &*riid;
        let pv: *const ::core::ffi::c_void = if riid == &::com::interfaces::iunknown::IID_IUNKNOWN
            || <IAntimalwareProvider as ::com::Interface>::is_iid_in_inheritance_chain(riid)
        {
            &self.__0_IAntimalwareProvider as *const _ as *const ::core::ffi::c_void
        } else {
            *ppv = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return ::com::sys::E_NOINTERFACE;
        };
        *ppv = pv as *mut ::core::ffi::c_void;
        self.AddRef();
        ::com::sys::NOERROR
    }
    pub fn query_interface<T: ::com::Interface>(
        self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
    ) -> Option<T> {
        let mut result = None;
        let hr = unsafe { self.QueryInterface(&T::IID, &mut result as *mut _ as _) };
        if ::com::sys::FAILED(hr) {
            return None;
        }
        if true {
            if !result.is_some() {
                {
                    ::std::rt::begin_panic(
                        "Successful call to query_interface yielded a null pointer",
                    );
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
    <IAntimalwareProvider as ::com::Interface>::VTable = {
    type IAntimalwareProviderVTable = <IAntimalwareProvider as ::com::Interface>::VTable;
    IAntimalwareProviderVTable {
        parent: {
            type IUknownVTable = <::com::interfaces::IUnknown as ::com::Interface>::VTable;
            unsafe extern "system" fn AddRef(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
            ) -> u32 {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProvider,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                munged.AddRef()
            }
            unsafe extern "system" fn Release(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
            ) -> u32 {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProvider,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                let new_ref_count = ::com::refcounting::release(&munged.__refcnt);
                if new_ref_count == 0 {
                    munged.drop_inner();
                }
                new_ref_count
            }
            unsafe extern "system" fn QueryInterface(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
                riid: *const ::com::sys::IID,
                ppv: *mut *mut ::core::ffi::c_void,
            ) -> ::com::sys::HRESULT {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProvider,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                munged.QueryInterface(riid, ppv)
            }
            IUknownVTable {
                AddRef,
                Release,
                QueryInterface,
            }
        },
        Scan: {
            #[allow(non_snake_case)]
            unsafe extern "system" fn Scan(
                this: ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                stream: <*mut c_void as ::com::AbiTransferable>::Abi,
                result: <*mut u8 as ::com::AbiTransferable>::Abi,
            ) -> HRESULT {
                let this = this.as_ptr().sub(0usize);
                let this =
                    ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(
                        this as *mut _ as *mut RustAmsiProvider,
                    ));
                let stream = <*mut c_void as ::com::AbiTransferable>::from_abi(stream);
                let result = <*mut u8 as ::com::AbiTransferable>::from_abi(result);
                RustAmsiProvider::Scan(&this, stream, result)
            }
            Scan
        },
        CloseSession: {
            #[allow(non_snake_case)]
            unsafe extern "system" fn CloseSession(
                this: ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                session: <u64 as ::com::AbiTransferable>::Abi,
            ) {
                let this = this.as_ptr().sub(0usize);
                let this =
                    ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(
                        this as *mut _ as *mut RustAmsiProvider,
                    ));
                let session = <u64 as ::com::AbiTransferable>::from_abi(session);
                RustAmsiProvider::CloseSession(&this, session)
            }
            CloseSession
        },
        DisplayName: {
            #[allow(non_snake_case)]
            unsafe extern "system" fn DisplayName(
                this: ::core::ptr::NonNull<::core::ptr::NonNull<IAntimalwareProviderVTable>>,
                f: <*mut u16 as ::com::AbiTransferable>::Abi,
            ) -> HRESULT {
                let this = this.as_ptr().sub(0usize);
                let this =
                    ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(
                        this as *mut _ as *mut RustAmsiProvider,
                    ));
                let f = <*mut u16 as ::com::AbiTransferable>::from_abi(f);
                RustAmsiProvider::DisplayName(&this, f)
            }
            DisplayName
        },
    }
};
impl<'a> ::core::convert::From<&'a RustAmsiProvider> for IAntimalwareProvider {
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
    __0_IClassFactory: &'static <::com::interfaces::IClassFactory as ::com::Interface>::VTable,
    __refcnt: ::core::sync::atomic::AtomicU32,
}
impl RustAmsiProviderClassFactory {
    #[doc = r" Allocate the class casting it to the supplied interface"]
    #[doc = r""]
    #[doc = r" This allocates the class on the heap and pins it. This is because COM classes"]
    #[doc = r" must have a stable location in memory. Once a COM class is instantiated somewhere"]
    #[doc = r" it must stay there."]
    pub fn allocate() -> ::com::production::ClassAllocation<Self> {
        let instance = RustAmsiProviderClassFactory {
            __0_IClassFactory: &RustAmsiProviderClassFactory__IClassFactory_VTABLE,
            __refcnt: ::core::sync::atomic::AtomicU32::new(1),
        };
        let instance = ::com::alloc::boxed::Box::pin(instance);
        ::com::production::ClassAllocation::new(instance)
    }
    #[allow(non_snake_case)]
    unsafe fn CreateInstance(
        &self,
        aggr: *mut ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
        riid: *const ::com::sys::IID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::com::sys::HRESULT {
        if !!riid.is_null() {
            {
                ::std::rt::begin_panic("iid passed to CreateInstance was null");
            }
        };
        if !aggr.is_null() {
            return ::com::sys::CLASS_E_NOAGGREGATION;
        }
        let instance = RustAmsiProvider::allocate();
        instance.QueryInterface(riid, ppv)
    }
    #[allow(non_snake_case)]
    unsafe fn LockServer(&self, _increment: com::sys::BOOL) -> com::sys::HRESULT {
        ::com::sys::S_OK
    }
    pub unsafe fn AddRef(self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>) -> u32 {
        ::com::refcounting::addref(&self.__refcnt)
    }
    #[inline(never)]
    pub unsafe fn QueryInterface(
        self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
        riid: *const ::com::sys::IID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::com::sys::HRESULT {
        let riid = &*riid;
        let pv: *const ::core::ffi::c_void = if riid == &::com::interfaces::iunknown::IID_IUNKNOWN
            || <::com::interfaces::IClassFactory as ::com::Interface>::is_iid_in_inheritance_chain(
                riid,
            ) {
            &self.__0_IClassFactory as *const _ as *const ::core::ffi::c_void
        } else {
            *ppv = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return ::com::sys::E_NOINTERFACE;
        };
        *ppv = pv as *mut ::core::ffi::c_void;
        self.AddRef();
        ::com::sys::NOERROR
    }
    pub fn query_interface<T: ::com::Interface>(
        self: &::core::pin::Pin<::com::alloc::boxed::Box<Self>>,
    ) -> Option<T> {
        let mut result = None;
        let hr = unsafe { self.QueryInterface(&T::IID, &mut result as *mut _ as _) };
        if ::com::sys::FAILED(hr) {
            return None;
        }
        if true {
            if !result.is_some() {
                {
                    ::std::rt::begin_panic(
                        "Successful call to query_interface yielded a null pointer",
                    );
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
    <::com::interfaces::IClassFactory as ::com::Interface>::VTable = {
    type IClassFactoryVTable = <::com::interfaces::IClassFactory as ::com::Interface>::VTable;
    IClassFactoryVTable {
        parent: {
            type IUknownVTable = <::com::interfaces::IUnknown as ::com::Interface>::VTable;
            unsafe extern "system" fn AddRef(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
            ) -> u32 {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProviderClassFactory,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                munged.AddRef()
            }
            unsafe extern "system" fn Release(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
            ) -> u32 {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProviderClassFactory,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                let new_ref_count = ::com::refcounting::release(&munged.__refcnt);
                if new_ref_count == 0 {
                    munged.drop_inner();
                }
                new_ref_count
            }
            unsafe extern "system" fn QueryInterface(
                this: ::core::ptr::NonNull<
                    ::core::ptr::NonNull<<::com::interfaces::IUnknown as ::com::Interface>::VTable>,
                >,
                riid: *const ::com::sys::IID,
                ppv: *mut *mut ::core::ffi::c_void,
            ) -> ::com::sys::HRESULT {
                let munged = this.as_ptr().sub(0usize);
                let munged = ::com::production::ClassAllocation::from_raw(
                    munged as *mut _ as *mut RustAmsiProviderClassFactory,
                );
                let mut munged = ::core::mem::ManuallyDrop::new(munged);
                munged.QueryInterface(riid, ppv)
            }
            IUknownVTable {
                AddRef,
                Release,
                QueryInterface,
            }
        },
        CreateInstance: {
            #[allow(non_snake_case)]
            unsafe extern "system" fn CreateInstance(
                this: ::core::ptr::NonNull<::core::ptr::NonNull<IClassFactoryVTable>>,
                aggr: <*mut ::core::ptr::NonNull<
                    <::com::interfaces::IUnknown as ::com::Interface>::VTable,
                > as ::com::AbiTransferable>::Abi,
                riid: <*const ::com::sys::IID as ::com::AbiTransferable>::Abi,
                ppv: <*mut *mut ::core::ffi::c_void as ::com::AbiTransferable>::Abi,
            ) -> ::com::sys::HRESULT {
                let this = this.as_ptr().sub(0usize);
                let this =
                    ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(
                        this as *mut _ as *mut RustAmsiProviderClassFactory,
                    ));
                let aggr = <*mut ::core::ptr::NonNull<
                    <::com::interfaces::IUnknown as ::com::Interface>::VTable,
                > as ::com::AbiTransferable>::from_abi(aggr);
                let riid = <*const ::com::sys::IID as ::com::AbiTransferable>::from_abi(riid);
                let ppv = <*mut *mut ::core::ffi::c_void as ::com::AbiTransferable>::from_abi(ppv);
                RustAmsiProviderClassFactory::CreateInstance(&this, aggr, riid, ppv)
            }
            CreateInstance
        },
        LockServer: {
            #[allow(non_snake_case)]
            unsafe extern "system" fn LockServer(
                this: ::core::ptr::NonNull<::core::ptr::NonNull<IClassFactoryVTable>>,
                _increment: <com::sys::BOOL as ::com::AbiTransferable>::Abi,
            ) -> com::sys::HRESULT {
                let this = this.as_ptr().sub(0usize);
                let this =
                    ::core::mem::ManuallyDrop::new(::com::production::ClassAllocation::from_raw(
                        this as *mut _ as *mut RustAmsiProviderClassFactory,
                    ));
                let _increment = <com::sys::BOOL as ::com::AbiTransferable>::from_abi(_increment);
                RustAmsiProviderClassFactory::LockServer(&this, _increment)
            }
            LockServer
        },
    }
};
impl<'a> ::core::convert::From<&'a RustAmsiProviderClassFactory>
    for ::com::interfaces::IClassFactory
{
    fn from(class: &'a RustAmsiProviderClassFactory) -> Self {
        unsafe {
            ::com::refcounting::addref(&class.__refcnt);
            ::core::mem::transmute(&class.__0_IClassFactory)
        }
    }
}
