#![allow(bad_style)]

pub use winapi::*;

#[macro_use]
extern crate winapi;

pub fn hello_world() -> winapi::HRESULT {
    println!("Hello, World!");
    winapi::S_OK
}

//"56f90ba7-54a6-001e-c419-0c8b60821376"
DEFINE_GUID!(IID_IDkmCallStackFilter, 0x56f90ba7, 0x54a6, 0x001e, 0xc4, 0x19, 0x0c, 0x8b, 0x60, 0x82, 0x13, 0x76);
RIDL!(
    interface IDkmCallStackFilter(IDkmCallStackFilterVtbl): IUnknown(IUnknownVtbl) {
        fn FilterNextFrame(&mut self, stackContext: *mut NativeDkmStackContext, input: *mut c_void, result: *mut c_void) -> HRESULT
    }
);

//"39738b1d-2e90-4164-a21c-749be02f9a96"
DEFINE_GUID!(IID_IDkmDisposableDataItem, 0x39738b1d, 0x2e90, 0x4164, 0xa2, 0x1c, 0x74, 0x9b, 0xe0, 0x2f, 0x9a, 0x96);
RIDL!(
    interface IDkmDisposableDataItem(IDkmDisposableDataItemVtbl): IUnknown(IUnknownVtbl) {
        fn OnClose(&mut self) -> HRESULT
    }
);

#[repr(C)]
pub struct NativeDkmDataItem {
    pub pValue: *const IUnknown,
    pub Id: GUID
}

#[repr(C)]
struct NativeCriticalSection {
    debugInfo: usize,
    lockCount: i32,
    recursionCount: i32,
    owningThread: usize,
    lockSemaphore: usize,
    spinCount: usize
}

#[repr(C)]
struct NativeXapiDispatcherObjectBase {
    vtable: IUnknown,
    refCount: i32,

    flags: i32, // TODO: enum or bitflags for this
    typeId: REFGUID,
    objectGCHandle: usize 
}

#[repr(C)]
struct NativeXapiDataContainer {
    nativeBase: NativeXapiDispatcherObjectBase,

    dataConatinerMap: usize,
    creator: usize,
    createEventPosition: usize,
    lock: NativeCriticalSection
}

#[repr(C)]
struct NativeDkmStackContext_ExtendedData {
    asyncContext: usize,
    operation: i32
}

#[repr(C)]
pub struct DkmFrameFormatOptions {
    argumentFlags: i32,
    frameNameFormat: i32,
    evaluationFlags: i32,
    timeout: u32,
    radix: u32
}

#[repr(C)]
pub struct NativeDkmStackContext {
    baseClass: NativeXapiDataContainer,

    inspectionSession: usize,
    thread: usize,
    filterOptions: i32,
    formatOptions: DkmFrameFormatOptions,
    threadContext: c_void,
    uniqueId: GUID,
    extendedData: *const NativeDkmStackContext_ExtendedData
}

pub struct DkmStackContext {
    pub native: *mut NativeDkmStackContext
}

impl DkmStackContext {

}

impl Clone for DkmStackContext {
    fn clone(&self) -> DkmStackContext {
        if !self.native.is_null() {
            unsafe { (*self.native).baseClass.nativeBase.vtable.AddRef(); }
        }
        DkmStackContext { native: self.native }
    }
}

// impl Drop for DkmStackContext {
//     fn drop(&mut self) {
//         if !self.native.is_null() {
//             unsafe { (*self.native).baseClass.nativeBase.vtable.Release(); }
//         }
//     }
// }

// #[link(name = "vsdebugeng")]
// extern "system" {
//     #[no_mangle]
//     pub fn DkmAllocBytes(bytes: usize, ppMemory: *mut *mut c_void);
// }

#[link(name = "kernel32")]
extern "system" {
    pub fn GlobalAlloc(flags: u32, bytes: usize) -> *mut c_void;
    pub fn GlobalFree(hMem: c_void) -> *mut c_void;
}