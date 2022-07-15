#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use windows::*;
use bindings::*;
use bindings::{
    Windows::Win32::{
        Foundation::{
            BOOL,
            CLASS_E_CLASSNOTAVAILABLE,
            E_NOINTERFACE,
            E_NOTIMPL,
            S_OK,
            E_INVALIDARG,
        },
        System::Com::{
            IClassFactory, IClassFactory_abi
        }
    }
};

static COMPONENT_GUID: Guid = Guid::from_values(
    0x9010c723, 
    0x91, 
    0x47af, 
    [0xb2, 0x87, 0x5d, 0x52, 0x9d, 0x29, 0x10, 0x7b]
);

#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: *const Guid, riid: *const Guid, ppv: *mut *mut std::ffi::c_void) -> HRESULT {
    if *rclsid == COMPONENT_GUID {
        let unk: IUnknown = ComponentClassFactory().into();
        unk.query(riid, ppv)
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(Windows::Win32::System::Com::IClassFactory)]
struct ComponentClassFactory();

impl ComponentClassFactory {
    unsafe fn CreateInstance(&self, outer: &Option<IUnknown>, riid: *const Guid, ppvojbect: *mut RawPtr) -> HRESULT {
        if outer.is_none() {
            let unk: IUnknown = MyFrameFilter().into();
            unk.query(riid, ppvojbect)
        } else {
            E_INVALIDARG
        }
    }

    fn LockServer(&self, flock: BOOL) -> HRESULT {
        E_NOTIMPL
    }
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

struct MyFrameFilter();

impl MyFrameFilter {
    fn FilterNextFrame(&self, context: &RustyDkmStackContext, input: &Option<RustyDkmStackWalkFrame>) -> Option<Vec<RustyDkmStackWalkFrame>> {

        let thread = context.Thread();

        let cloned: RustyDkmThread = thread.clone();
        
        //let next_context: RustyDkmStackContext = context.clone();

        match input {
            Some(_) => None,
            None => {
                let frame = RustyDkmStackWalkFrame::create(
                    thread,
                    &None,
                    0,
                    0,
                    vsdebugeng::CallStack::DkmStackWalkFrameFlags::None,
                    "[hello from rust]",
                    &None,
                );

                Some(vec![frame])
            }
        }
    }
}

impl ::std::convert::From<MyFrameFilter> for ::windows::IUnknown {
    fn from(implementation: MyFrameFilter) -> Self {
        let com = MyFrameFilter_box::new(implementation);
        unsafe {
            let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
            ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(
                &mut (*ptr).identity_vtable as *mut _ as _,
            ))
        }
    }
}

#[repr(C)]
struct MyFrameFilter_box {
    identity_vtable: *const IUnknown_abi,
    vtables: (*const IDkmCallStackFilter_abi,),
    implementation: MyFrameFilter,
    count: WeakRefCount,
}

impl MyFrameFilter_box {
    const VTABLES: (IDkmCallStackFilter_abi,) = 
        (IDkmCallStackFilter_abi(
            Self::QueryInterface_abi0,
            Self::AddRef_abi0,
            Self::Release_abi0,
            Self::abi1,
        ),);
    const IDENTITY_VTABLE: IUnknown_abi = IUnknown_abi(
        Self::identity_query_interface,
        Self::identity_add_ref,
        Self::identity_release,
    );
    const IID0: Guid = <IDkmCallStackFilter as Interface>::IID;

    fn new(implementation: MyFrameFilter) -> Self {
        Self {
            identity_vtable: &Self::IDENTITY_VTABLE,
            vtables: (&Self::VTABLES.0,),
            implementation,
            count: WeakRefCount::new(),
        }
    }

    fn QueryInterface(
        &mut self,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        unsafe {
            *interface = if iid == &<IUnknown as Interface>::IID
            {
                &mut self.identity_vtable as *mut _ as _
            } else if iid == &Self::IID0 {
                &mut self.vtables.0 as *mut _ as _
            } else {
                std::ptr::null_mut()
            };
            if !(*interface).is_null() {
                self.count.add_ref();
                return HRESULT(0);
            }
            E_NOINTERFACE
        }
    }
    fn AddRef(&mut self) -> u32 {
        self.count.add_ref()
    }
    fn Release(&mut self) -> u32 {
        let remaining = self.count.release();
        if remaining == 0 { 
            unsafe { Box::from_raw(self); }
        }
        remaining
    }

    unsafe extern "system" fn identity_query_interface(
        this: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = (this as *mut RawPtr).sub(0) as *mut Self;
        (*this).QueryInterface(iid, interface)
    }

    unsafe extern "system" fn identity_add_ref(this: RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(0) as *mut Self;
        (*this).AddRef()
    }

    unsafe extern "system" fn identity_release(this: RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(0) as *mut Self;
        (*this).Release()
    }

    unsafe extern "system" fn QueryInterface_abi0(
        this: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        (*this).QueryInterface(iid, interface)
    }

    unsafe extern "system" fn AddRef_abi0(this: RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        (*this).AddRef()
    }

    unsafe extern "system" fn Release_abi0(this: RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        (*this).Release()
    }

    unsafe extern "system" fn abi1(
        this: RawPtr, 
        pStackContext: *mut vsdebugeng::CallStack::DkmStackContext, 
        pInput: *mut vsdebugeng::CallStack::DkmStackWalkFrame, 
        pResult: *mut DkmArray<*mut vsdebugeng::CallStack::DkmStackWalkFrame>
    ) -> HRESULT {

        let this = (this as *mut RawPtr).sub(1) as *mut Self;

        IDkmStackCallStackFilter_FilterNextFrame(
            pStackContext, 
            pInput, 
            pResult,
            |context, input| { (*this).implementation.FilterNextFrame(context, input) }
        )
    }
}

unsafe fn IDkmStackCallStackFilter_FilterNextFrame(
    pStackContext: *mut vsdebugeng::CallStack::DkmStackContext, 
    pInput: *mut vsdebugeng::CallStack::DkmStackWalkFrame, 
    pResult: *mut DkmArray<*mut vsdebugeng::CallStack::DkmStackWalkFrame>,
    callback: impl FnOnce(&RustyDkmStackContext, &Option<RustyDkmStackWalkFrame>) -> Option<Vec<RustyDkmStackWalkFrame>>
) -> HRESULT {
    let context = std::mem::transmute::<_, &RustyDkmStackContext>(&pStackContext);
    let input = std::mem::transmute::<_, &Option<RustyDkmStackWalkFrame>>(&pInput);
    let result = callback(context, input);

    match result {
        None=> HRESULT(1),
        Some(frames) => {
            ProcDkmAlloc(std::mem::size_of::<RawPtr>() * frames.len(), &mut (*pResult).members as *mut _ as _);
            (*pResult).length = frames.len() as u32;

            let mut n = 0;
            while n < frames.len() {
                *((*pResult).members.add(n)) = std::mem::transmute_copy(&frames[n]);
                n += 1;
            }
            S_OK
        }
    }
}

#[repr(transparent)]
struct RustyDkmObjectBase(std::ptr::NonNull<vsdebugeng::NativeXapiDispatcherObjectBase>);

impl RustyDkmObjectBase {
    unsafe fn vtable(&self) -> &IUnknown_abi {
        let this: RawPtr = std::mem::transmute_copy(self);
        &(*(*(this as *mut *mut _) as *mut _))
    }
}

impl Clone for RustyDkmObjectBase {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().1)(std::mem::transmute_copy(self));
        }
        
        Self(self.0)
    }
}

impl Drop for RustyDkmObjectBase {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().2)(std::mem::transmute_copy(self));
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
struct RustyDkmStackContext(RustyDkmObjectBase);

impl RustyDkmStackContext {
    fn Thread(&self) -> &RustyDkmThread {
        unsafe {
            let context: *mut vsdebugeng::CallStack::DkmStackContext = std::mem::transmute_copy(self);
            let thread = &(*context).Thread;
            std::mem::transmute_copy::<_, &RustyDkmThread>(&thread)
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
struct RustyDkmStackWalkFrame(RustyDkmObjectBase);
// impl RustyDkmStackWalkFrame {
//     fn Thread(&self) -> &RustyDkmThread {
//         unsafe {
//             std::mem::transmute::<_, &RustyDkmThread>(&(*(self.0.as_ptr())).Thread)
//         }
//     }

//     fn InstructionAddress(&self) -> &Option<RustyDkmInstructionAddress> {
//         unsafe {
//             std::mem::transmute::<_, &Option<RustyDkmInstructionAddress>>(&(*(self.0.as_ptr())).InstructionAddress)
//         }  
//     }
// }

#[repr(transparent)]
#[derive(Clone)]
struct RustyDkmThread(RustyDkmObjectBase);

#[repr(transparent)]
#[derive(Clone)]
struct RustyDkmInstructionAddress(RustyDkmObjectBase);

#[repr(transparent)]
#[derive(Clone)]
struct RustyDkmFrameRegisters(RustyDkmObjectBase);

impl RustyDkmStackWalkFrame {
    fn create(
        thread: &RustyDkmThread,
        instruction_address: &Option<RustyDkmInstructionAddress>,
        frame_base: u64,
        frame_size: u32,
        flags: vsdebugeng::CallStack::DkmStackWalkFrameFlags,
        description: &str,
        registers: &Option<RustyDkmFrameRegisters>,
    ) -> RustyDkmStackWalkFrame {
        unsafe {
            let mut native_string: *mut vsdebugeng::DkmString = std::ptr::null_mut() as *mut _;
            ProcDkmString1(
                65001,
                description.as_ptr(),
                description.len(),
                &mut native_string
            );

            let native_instruction_address: *mut vsdebugeng::DkmInstructionAddress =
                match instruction_address {
                    None => std::mem::zeroed(),
                    Some(_instruction_address) => std::mem::transmute_copy(instruction_address),
                };

            let native_registers: *mut vsdebugeng::CallStack::DkmFrameRegisters = 
                match registers {
                    None => std::mem::zeroed(),
                    Some(_registers) => std::mem::transmute_copy(registers),
                };

            let mut native_result: *mut vsdebugeng::CallStack::DkmStackWalkFrame = std::ptr::null_mut();
            ProcA0BA43B79BBE61B6ED073DE327837C99(
                std::mem::transmute_copy(thread),
                native_instruction_address,
                frame_base,
                frame_size,
                flags,
                native_string,
                native_registers,
                std::ptr::null_mut(),
                &mut native_result
            );

            std::mem::transmute(native_result)
        }
    }
}

// #[repr(C)]
// struct MyFrameFilter {
//     vtable: *const IDkmCallStackFilter_abi,
//     count: u32
// }

// impl From<MyFrameFilter> for IUnknown {
//     fn from(filter: MyFrameFilter) -> Self {
//         unsafe {
//             let ptr = Box::into_raw(Box::new(filter));
//             std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).vtable as *mut _ as _))
//         }
//     }
// }

// impl MyFrameFilter {
//     const VTABLE: IDkmCallStackFilter_abi = IDkmCallStackFilter_abi(
//         Self::QueryInterface_abi,
//         Self::AddRef_abi,
//         Self::Release_abi,
//         Self::FilterNextFrame_abi,
//     );

//     pub fn new() -> MyFrameFilter {
//         MyFrameFilter {
//             vtable: &Self::VTABLE,
//             count: 1
//         }
//     }

//     pub fn AddRef(&mut self) -> u32 {
//         self.count += 1;
//         self.count
//     }
    
//     pub fn Relese(&mut self) -> u32 {
//         self.count -= 1;
//         let remaining = self.count;
//         if remaining == 0 {
//             unsafe {
//                 Box::from_raw(self);
//             }
//         }

//         remaining
//     }

//     unsafe extern "system" fn QueryInterface_abi(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT {
//         if *iid == <IUnknown as Interface>::IID || *iid == <IDkmCallStackFilter as Interface>::IID {
//             *interface = this;

//             let this = this as *mut Self;
//             (*this).AddRef();

//             return S_OK;
//         }
        
//         E_NOINTERFACE
//     }

//     unsafe extern "system" fn AddRef_abi(this: RawPtr) -> u32 {
//         let this = this as *mut Self;
//         (*this).AddRef()
//     }

//     unsafe extern "system" fn Release_abi(this: RawPtr) -> u32 {
//         let this = this as *mut Self;
//         (*this).Relese()
//     }

//     unsafe extern "system" fn FilterNextFrame_abi(this: RawPtr, pStackContext: *mut vsdebugeng::CallStack::DkmStackContext, pInput: *mut vsdebugeng::CallStack::DkmStackWalkFrame, pResult: *mut DkmArray<*mut vsdebugeng::CallStack::DkmStackWalkFrame>) -> HRESULT {
//         if pInput.is_null() {

//             let mut string: *mut vsdebugeng::DkmString = std::ptr::null_mut();

//             let message = "[hello from rust]";
//             ProcDkmString1(
//                 65001,
//                 message.as_ptr(),
//                 message.len(),
//                 &mut string
//             );

//             let mut frame: *mut vsdebugeng::CallStack::DkmStackWalkFrame = std::ptr::null_mut();

//             ProcA0BA43B79BBE61B6ED073DE327837C99(
//                 (*pStackContext).Thread,
//                 std::ptr::null_mut(),
//                 0,
//                 0,
//                 vsdebugeng::CallStack::DkmStackWalkFrameFlags::None,
//                 string,
//                 std::ptr::null_mut(),
//                 std::ptr::null_mut(),
//                 &mut frame
//             );

//             ProcDkmAlloc(std::mem::size_of::<RawPtr>(), &mut (*pResult).members as *mut _ as _);
//             (*pResult).length = 1;
            
//             *(*pResult).members = frame;

//             return S_OK;
//         }

//         return HRESULT(1);
        
//         // (*(*pInput).native_base.native_base.vtable).1(pInput as *mut _);

//         // ProcDkmAlloc(std::mem::size_of::<RawPtr>(), &mut (*pResult).members as *mut _ as _);
//         // (*pResult).length = 1;
        
//         // *(*pResult).members = pInput as *mut _;

//         S_OK
//     }
// }

#[repr(C)]
pub struct IUnknown_abi(
    pub unsafe extern "system" fn(this: RawPtr, &Guid, interface: *mut RawPtr) -> HRESULT,
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