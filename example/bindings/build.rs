fn main() {
    windows::build! {
        Windows::Win32::Foundation::{
            BOOL,
            CLASS_E_CLASSNOTAVAILABLE,
            E_NOINTERFACE,
            E_NOTIMPL,
            S_OK,
        },
        Windows::Win32::System::Com::{
            CoInitializeEx,
            IClassFactory,
        },
    };
}