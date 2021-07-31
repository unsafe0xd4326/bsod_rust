#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use {
    winapi::shared::ntdef::NTSTATUS,
    winapi::shared::ntdef::ULONG,
    winapi::shared::ntdef::BOOLEAN,
    winapi::shared::ntdef::PBOOLEAN,
    winapi::shared::ntdef::LONG,
    winapi::shared::basetsd::PULONG_PTR,
    winapi::shared::ntdef::PULONG,
    winapi::shared::ntstatus::STATUS_ASSERTION_FAILURE
};


#[link(name = "ntdll")]
extern {
  fn RtlAdjustPrivilege(Privilege: ULONG, Enable: BOOLEAN, CurrentThread: BOOLEAN , OldValue: PBOOLEAN) -> NTSTATUS ;
  fn NtRaiseHardError(ErrorStatus: LONG, NumberOfParameters: ULONG, UnicodeStringParameterMask: ULONG, Parameters: PULONG_PTR, ValidResponseOptions: ULONG, Response: PULONG) -> NTSTATUS; 
}

// cargo run --release --target x86_64-pc-windows-msvc
fn main (){
    unsafe {
        println!("hello");
        let mut b : BOOLEAN = 1;
        let mut response : ULONG = 0 ;
        RtlAdjustPrivilege(19, 1, 0, &mut b);
        NtRaiseHardError(STATUS_ASSERTION_FAILURE,0,0,0 as *mut _,6, &mut response);
        println!("{} {} ",b,response );
    }
}