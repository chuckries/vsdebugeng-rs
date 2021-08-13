fn main() {
    
}

// use windows::*;

// use bindings::Windows::Win32::Foundation::*;
// use bindings::Windows::Win32::System::Com::*;

// use std::ffi::c_void;

// fn main() -> Result<()> {
//     intialize_dispatcher()?;

//     Ok(())
// }

// fn intialize_dispatcher() -> Result<()> {
//     unsafe {
//         CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;

//         let session_callback: *const ISessionRemotingCallback_abi = &ISessionRemotingCallback_vtable;

//         DkmDllEnsureInitialized(
//             std::ptr::null(),
//             1033,
//             &session_callback,
//             std::ptr::null(),
//             std::ptr::null()).ok()?;

//         DkmDllUninitialize().ok()?;

//         Ok(())
//     }
// }

// #[link(name = "D:\\source\\Concord\\bin\\Debug\\SDK\\import-lib\\x64\\vsdebugeng")]
// extern "stdcall" {
//     fn DkmDllEnsureInitialized(
//         registry_root: *const u16, 
//         locale: u16, 
//         callback: *const *const ISessionRemotingCallback_abi, 
//         service_provider: *const c_void, 
//         remote_options: *const c_void) -> HRESULT;

//     fn DkmDllUninitialize() -> HRESULT;
// }

// #[repr(C)]
// struct IUnknown_abi(
//     pub unsafe extern "system" fn(this: RawPtr, iid: *const Guid, interface: *mut RawPtr) -> HRESULT,
//     pub unsafe extern "system" fn(this: RawPtr) -> u32,
//     pub unsafe extern "system" fn(this: RawPtr) -> u32,
// );

// #[repr(C)]
// struct ISessionRemotingCallback_abi(
//     pub unsafe extern "system" fn(this: RawPtr, iid: *const Guid, interface: *mut RawPtr) -> HRESULT,
//     pub unsafe extern "system" fn(this: RawPtr) -> u32,
//     pub unsafe extern "system" fn(this: RawPtr) -> u32,
//     pub unsafe extern "system" fn(this: RawPtr, connection: RawPtr, reason: u32) -> HRESULT,
// );

// static ISessionRemotingCallback_vtable: ISessionRemotingCallback_abi = ISessionRemotingCallback_abi {
//     0: query_interface,
//     1: addref,
//     2: release,
//     3: on_disconnect
// };

// extern "system" fn query_interface(this: RawPtr, iid: *const Guid, interace: *mut RawPtr) -> HRESULT {
//     E_NOINTERFACE
// }

// extern "system" fn addref(this: RawPtr) -> u32 {
//     1
// }

// extern "system" fn release(this: RawPtr) -> u32 {
//     1
// }

// extern "system" fn on_disconnect(this: RawPtr, connection: RawPtr, reason: u32) -> HRESULT {
//     HRESULT(0)
// }