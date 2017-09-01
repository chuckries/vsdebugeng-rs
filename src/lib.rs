#![allow(bad_style)]

use vsdebugeng::*;
use std::ptr;

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

#[repr(C)]
pub struct DataItem<T: DkmDataItem> {
    vtbl: IDkmDisposableDataItem,
    cRef: u32,
    data: T
}

extern "system" fn DataItemQueryInterface<T>(pUnk: LPUNKNOWN, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
    S_OK
}

extern "system" fn DataItemAddRef<T>(pUnk: LPUNKNOWN) -> ULONG {
    1
}

extern "system" fn DataItemRelease<T>(pUnk: LPUNKNOWN) -> ULONG {
    1
}

unsafe extern "system" fn DataItemOnClose<T: DkmDataItem>(pUnk: *mut IDkmDisposableDataItem) -> HRESULT {
    let imp: &mut DataItem<T> = &mut *(pUnk as *mut DataItem<T>);
    let hr = imp.data.OnClose();
    hr
}

impl<T: DkmDataItem> DataItem<T> {
    fn new(data: T) -> DataItem<T> {
        DataItem {
            vtbl: IDkmDisposableDataItem {
                lpVtbl: &IDkmDisposableDataItemVtbl {
                    parent: IUnknownVtbl {
                        QueryInterface: DataItemQueryInterface::<T>,
                        AddRef: DataItemAddRef::<T>,
                        Release: DataItemRelease::<T>
                    },
                    OnClose: DataItemOnClose::<T>
                } as *const IDkmDisposableDataItemVtbl
            },
            cRef: 1,
            data: data
        }
    }
}

pub trait DkmDataItem {
    fn iid() -> IID;
    fn OnClose(&mut self) -> HRESULT {
        S_OK
    }
}

fn ProcDkmSetDataItem(item: *const NativeDkmDataItem) -> HRESULT {
    S_OK
}

fn ProcDkmGetDataItem(id: REFGUID, ppUnk: *mut *mut IUnknown) -> HRESULT {
    S_OK
}

fn SetDataItem<T: DkmDataItem>(item: T) -> HRESULT {
    let pDataItem = Box::into_raw(Box::new(DataItem::new(item))) as *mut IUnknown;

    let hr = ProcDkmSetDataItem(&NativeDkmDataItem {
        pValue: pDataItem,
        Id: T::iid()
    });

    hr
}

fn GetDataItem<'a, T: DkmDataItem>() -> Option<&'a T> {
    let mut pUnk: *mut IUnknown = std::ptr::null_mut();
    let hr = ProcDkmGetDataItem(&T::iid(), &mut pUnk);

    if hr == S_OK {
        unsafe { Some(& *(pUnk as *mut T)) }
    } else {
        None
    }
}

pub struct MyDataItem {
    num: u32
}

DEFINE_GUID!(GUID_NULL, 0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);

impl DkmDataItem for MyDataItem {
    fn iid() -> IID {
        GUID_NULL
    }
}

#[test]
fn data_item_test() {
    let dataItem = MyDataItem {
        num: 4
    };

    let hr = SetDataItem(dataItem);
    assert_eq!(S_OK, hr);

    let otherDataItem = GetDataItem::<MyDataItem>();
}

struct A;

#[test]
fn test_option() {
    let a: A = A;

    let opt = Some(&a);

    let thing = opt.unwrap();
}