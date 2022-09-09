use std::ffi::CStr;
use std::os::raw::c_char;
use std::{slice, ptr};

#[no_mangle]
pub extern "C" fn print_version() {
    println!("count version 1.0.0");
}

#[no_mangle]
pub extern "C" fn count_characters(text: *const c_char) -> u64 {
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_str().expect("Unicode conversion failed.");
    text.chars().count().try_into().unwrap()
}

/// cbindgen:prefix-with-name
#[repr(C)]
#[derive(PartialEq)]
pub enum Command {
    Version,
    Bytes,
    Characters,
}

#[repr(C)]
pub struct Arguments {
    command: Command,
    filename: *const c_char,
}

#[no_mangle]
pub extern "C" fn parse_args(argc: usize, argv: *const *const c_char) -> Arguments {
    let arguments = unsafe { slice::from_raw_parts(argv, argc) };

    let command = arguments.get(1).copied().expect("Missing command.");
    let command = unsafe { CStr::from_ptr(command) }.to_str().unwrap();
    let command = match command {
        "version" => Command::Version,
        "bytes" => Command::Bytes,
        "characters" => Command::Characters,
        _ => panic!("Command not recognized: {command}")
    };

    let filename = arguments.get(2).copied();
    if command != Command::Version && filename.is_none() {
        panic!("Missing filename.");
    }
    let filename = filename.unwrap_or(ptr::null());

    Arguments { command, filename }
}
