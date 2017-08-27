#![allow(bad_style)]

use vsdebugeng::*;

extern crate vsdebugeng;

#[no_mangle]
pub extern "stdcall" fn hello_world() -> vsdebugeng::HRESULT {
    vsdebugeng::hello_world()
}

struct MyCallStackFilter {
    vtbl: IDkmCallStackFilter,
    cRef: ULONG,
    context: DkmStackContext
}

impl MyCallStackFilter {
    pub fn filter_next_frame(&mut self, stackContext: &DkmStackContext) -> HRESULT {

        let newContext: DkmStackContext = stackContext.clone();
        self.context = newContext;

        S_OK
    }
}

unsafe extern "system" fn MyCallStackFilterQueryInterface(This: *mut IUnknown, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
    *ppvObject = This as *mut c_void;
    S_OK
}

unsafe extern "system" fn MyCallStackFilterAddRef(This: *mut IUnknown) -> ULONG {
    1
}

unsafe extern "system" fn MyCallStackFilterRelease(This: *mut IUnknown) -> ULONG {
    1
}

unsafe extern "system" fn MyCallStackFilterFilterNextFrame(This: *mut IDkmCallStackFilter, stackContext: *mut NativeDkmStackContext, input: *mut c_void, result: *mut c_void) -> HRESULT {
    let l_stackContext: DkmStackContext = DkmStackContext { native: stackContext };
    
    let imp: &mut MyCallStackFilter = &mut *(This as *mut MyCallStackFilter);
    imp.filter_next_frame(&l_stackContext)
}

static MyCallStatckFilterVtblImpl: IDkmCallStackFilterVtbl = IDkmCallStackFilterVtbl {
    parent: IUnknownVtbl {
        QueryInterface: MyCallStackFilterQueryInterface,
        AddRef: MyCallStackFilterAddRef,
        Release: MyCallStackFilterRelease,
    },
    FilterNextFrame: MyCallStackFilterFilterNextFrame,
};

static mut myCallStackFilter: MyCallStackFilter = MyCallStackFilter {
    vtbl: IDkmCallStackFilter {
        lpVtbl: &MyCallStatckFilterVtblImpl as *const IDkmCallStackFilterVtbl
    },
    cRef: 1,
    context: DkmStackContext { native: 0 as *mut NativeDkmStackContext }
};

unsafe extern "system" fn IClassFactoryQueryInterface(This: *mut IUnknown, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
    *ppvObject = This as *mut c_void;
    S_OK
}

unsafe extern "system" fn IClassFactoryAddRef(This: *mut IUnknown) -> ULONG {
    1
}

unsafe extern "system" fn IClassFactoryRelease(This: *mut IUnknown) -> ULONG {
    1
}

unsafe extern "system" fn IClassFactoryCreateInstance(This: *mut IClassFactory, pUnkOuter: *mut IUnknown, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
    // let ptr = GlobalAlloc(0, std::mem::size_of::<MyCallStackFilter>()) as *mut MyCallStackFilter;
    // if !ptr.is_null() {
    //     *ptr = MyCallStackFilter {
    //         vtbl: IDkmCallStackFilter {
    //             lpVtbl: &MyCallStatckFilterVtblImpl as *const IDkmCallStackFilterVtbl
    //         },
    //         cRef: 1,
    //         context: DkmStackContext { native: std::ptr::null_mut() }
    //     };
    //     let hr = ptr.QueryInterface(riid, ppvObject);
    //     ptr.Release();
    //     hr
    // }
    // else {
    //     E_OUTOFMEMORY
    // }
    myCallStackFilter.vtbl.QueryInterface(riid, ppvObject)
}

unsafe extern "system" fn IClassFactoryLockServer(This: *mut IClassFactory, fLock: BOOL) -> HRESULT {
    E_NOTIMPL
}

static IClassFactoryVtblImpl: IClassFactoryVtbl = IClassFactoryVtbl {
    parent: IUnknownVtbl {
        QueryInterface: IClassFactoryQueryInterface,
        AddRef: IClassFactoryAddRef,
        Release: IClassFactoryRelease
    },
    CreateInstance: IClassFactoryCreateInstance,
    LockServer: IClassFactoryLockServer
};

static mut classFactory: IClassFactory = IClassFactory {
    lpVtbl: &IClassFactoryVtblImpl as *const IClassFactoryVtbl
};

#[no_mangle]
pub unsafe extern "system" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
    classFactory.QueryInterface(riid, ppvObject)
}