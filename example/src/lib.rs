use windows::{
    Abi,
    Guid,
    HRESULT,
    implement,
    Interface,
    IUnknown,
    RawPtr,
};

use bindings::*;
use bindings::{
    Windows::Win32::{
        Foundation::{
            BOOL,
            CLASS_E_CLASSNOTAVAILABLE,
            E_NOINTERFACE,
            E_NOTIMPL,
            S_OK,
        },
        System::Com::{
            IClassFactory, IClassFactory_abi
        }
    }
};

// {c}
static COMPONENT_GUID: Guid = Guid::from_values(
    0x9010c723, 
    0x91, 
    0x47af, 
    [0xb2, 0x87, 0x5d, 0x52, 0x9d, 0x29, 0x10, 0x7b]
);

#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: *const Guid, riid: *const Guid, ppv: *mut *const std::ffi::c_void) -> HRESULT {
    if *rclsid == COMPONENT_GUID {
        *ppv = &CLASS_FACTORY.vtable as *const _ as _;
        return S_OK;
    }

    return CLASS_E_CLASSNOTAVAILABLE
}

struct ClassFactory {
    vtable: *const IClassFactory_abi,
}

unsafe impl Sync for ClassFactory {}

static CLASS_FACTORY: ClassFactory = ClassFactory {
    vtable: &CLASS_FACTORY_VTABLE
};

static CLASS_FACTORY_VTABLE: IClassFactory_abi = 
    IClassFactory_abi(
        class_factory_query_interface,
        class_factory_addref,
        class_factory_release,
        class_factory_create_instance,
        class_factory_lock_server
    );

unsafe extern "system" fn class_factory_query_interface(this: RawPtr, riid: &Guid, ppv: *mut RawPtr) -> HRESULT {
    if *riid == <IUnknown as Interface>::IID || *riid == <IClassFactory as Interface>::IID {
        *ppv = this;
        return S_OK;
    }

    E_NOINTERFACE
}

unsafe extern "system" fn class_factory_addref(this: RawPtr) -> u32 {
    1
}

unsafe extern "system" fn class_factory_release(this: RawPtr) -> u32 {
    1
}

unsafe extern "system" fn class_factory_create_instance(this: RawPtr, pUnkOuter: RawPtr, riid: *const Guid, ppvObject: *mut RawPtr) -> HRESULT {
    if *riid == <IUnknown as Interface>::IID || *riid == <IDkmCallStackFilter as Interface>::IID {
        unsafe {
            let unk: IUnknown = MyFrameFilter::new().into();
            unk.vtable().1(unk.abi()); // something wrong with ref count
            *ppvObject = unk.abi();
            return S_OK;
        }
    }

    E_NOINTERFACE
}

unsafe extern "system" fn class_factory_lock_server(this: RawPtr, fLock: BOOL) -> HRESULT {
    E_NOTIMPL
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Debug)]
struct IDkmCallStackFilter(IUnknown);

unsafe impl Interface for IDkmCallStackFilter {
    type Vtable = IDkmCallStackFilter_abi;
    const IID: Guid = Guid::from_values(
        0x56f90ba7,
        0x54a6,
        0x001e,
        [0xc4, 0x19, 0x0c, 0x8b, 0x60, 0x82, 0x13, 0x76]
    );
}

#[repr(C)]
struct IDkmCallStackFilter_abi(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, pStackContext: *mut vsdebugeng::CallStack::DkmStackContext, pInput: *mut vsdebugeng::CallStack::DkmStackWalkFrame, pResult: *mut DkmArray<*mut vsdebugeng::CallStack::DkmStackWalkFrame>) -> HRESULT
);

#[repr(C)]
struct MyFrameFilter {
    vtable: *const IDkmCallStackFilter_abi,
    count: u32
}

impl From<MyFrameFilter> for IUnknown {
    fn from(filter: MyFrameFilter) -> Self {
        unsafe {
            let ptr = Box::into_raw(Box::new(filter));
            std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).vtable as *mut _ as _))
        }
    }
}

impl MyFrameFilter {
    const VTABLE: IDkmCallStackFilter_abi = IDkmCallStackFilter_abi(
        Self::QueryInterface_abi,
        Self::AddRef_abi,
        Self::Release_abi,
        Self::FilterNextFrame_abi,
    );

    pub fn new() -> MyFrameFilter {
        MyFrameFilter {
            vtable: &Self::VTABLE,
            count: 1
        }
    }

    pub fn AddRef(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
    
    pub fn Relese(&mut self) -> u32 {
        self.count -= 1;
        let remaining = self.count;
        if remaining == 0 {
            unsafe {
                Box::from_raw(self);
            }
        }

        remaining
    }

    unsafe extern "system" fn QueryInterface_abi(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT {
        if *iid == <IUnknown as Interface>::IID || *iid == <IDkmCallStackFilter as Interface>::IID {
            *interface = this;

            let this = this as *mut Self;
            (*this).AddRef();

            return S_OK;
        }
        
        E_NOINTERFACE
    }

    unsafe extern "system" fn AddRef_abi(this: RawPtr) -> u32 {
        let this = this as *mut Self;
        (*this).AddRef()
    }

    unsafe extern "system" fn Release_abi(this: RawPtr) -> u32 {
        let this = this as *mut Self;
        (*this).Relese()
    }

    unsafe extern "system" fn FilterNextFrame_abi(this: RawPtr, pStackContext: *mut vsdebugeng::CallStack::DkmStackContext, pInput: *mut vsdebugeng::CallStack::DkmStackWalkFrame, pResult: *mut DkmArray<*mut vsdebugeng::CallStack::DkmStackWalkFrame>) -> HRESULT {
        if pInput.is_null() {

            let mut string: *mut vsdebugeng::DkmString = std::ptr::null_mut();

            let message = "[hello from rust]";
            ProcDkmString1(
                65001,
                message.as_ptr(),
                message.len(),
                &mut string
            );

            let mut frame: *mut vsdebugeng::CallStack::DkmStackWalkFrame = std::ptr::null_mut();

            ProcA0BA43B79BBE61B6ED073DE327837C99(
                (*pStackContext).Thread,
                std::ptr::null_mut(),
                0,
                0,
                vsdebugeng::CallStack::DkmStackWalkFrameFlags::None,
                string,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut frame
            );

            ProcDkmAlloc(std::mem::size_of::<RawPtr>(), &mut (*pResult).members as *mut _ as _);
            (*pResult).length = 1;
            
            *(*pResult).members = frame;

            return S_OK;
        }

        return HRESULT(1);
        
        // (*(*pInput).native_base.native_base.vtable).1(pInput as *mut _);

        // ProcDkmAlloc(std::mem::size_of::<RawPtr>(), &mut (*pResult).members as *mut _ as _);
        // (*pResult).length = 1;
        
        // *(*pResult).members = pInput as *mut _;

        S_OK
    }
}

#[repr(C)]
pub struct IUnknown_abi(
    pub unsafe extern "system" fn(this: RawPtr, iid: *const Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
);

struct DkmArray<T> {
    members: *mut T,
    length: u32
}

#[link(name = "D:\\source\\Concord\\bin\\Debug\\SDK\\import-lib\\x64\\vsdebugeng")]
extern "system" {
    fn ProcDkmAlloc(bytes: usize, ppMemory: *mut RawPtr) -> HRESULT;
    fn ProcA0BA43B79BBE61B6ED073DE327837C99(
        thread: *const vsdebugeng::DkmThread, 
        instruction_address: *mut vsdebugeng::DkmInstructionAddress, 
        frame_base: u64, 
        frame_size: u32, 
        flags: vsdebugeng::CallStack::DkmStackWalkFrameFlags, 
        description: *mut vsdebugeng::DkmString, 
        registers: *mut vsdebugeng::CallStack::DkmFrameRegisters,
        annotation: *mut vsdebugeng::DkmReadOnlyCollection<*const vsdebugeng::CallStack::DkmStackWalkFrameAnnotation>, 
        ppCreatedObject: *mut *mut vsdebugeng::CallStack::DkmStackWalkFrame);
    fn ProcDkmString1(code_page: u32, multi_byte_string: *const u8, size: usize, ppString: *mut *mut vsdebugeng::DkmString);
}