extern crate winapi;

use winapi::{
    shared::{
        minwindef,
        minwindef::{BOOL, DWORD, HINSTANCE, LPVOID}
    },
    um::consoleapi,
};

/// Entry point which will be called by the system once the DLL has been loaded
/// in the target process. Declaring this function is optional.
///
/// # Safety
///
/// What you can safely do inside here is very limited, see the Microsoft documentation
/// about "DllMain". Rust also doesn't officially support a "life before main()",
/// though it is unclear what that that means exactly for DllMain.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => demo_init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn demo_init() {
    unsafe { consoleapi::AllocConsole() };
    println!("Hello from dll written in Rust!");
}

// ========= General stuff is above, FOnline-related stuff is below

#[cfg(feature = "server")]
mod mutual {
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "C" fn SERVER() {
        // FOnline needs this to check if this is correct dll for server
    }

    // Placeholder for Critter struct
    pub enum Critter{}

    pub type CritterMutual = Critter;
}

#[cfg(feature = "client")]
mod mutual {
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "C" fn CLIENT() {
        // FOnline needs this to check if this is correct dll for client
    }

    // Placeholder for CritterCl struct
    pub enum CritterCl{}

    pub type CritterMutual = CritterCl;
}

use mutual::CritterMutual;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn getParam_Strength(cr: *mut CritterMutual, _: u32) -> i32 {
    // Placeholder for getParam_Strength (it works)
	8
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn TestFuncRust() {
    println!("TestFuncRust!");
}
