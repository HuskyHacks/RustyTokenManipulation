use std::error::Error;
use std::env::{var, args};
use std::process::{Command, exit};
use std::ptr::null_mut;
use windows::core::{PCSTR, PSTR, PWSTR};
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE, GetLastError},
    Security::{
        ImpersonateLoggedOnUser, LogonUserA, LOGON32_LOGON_NEW_CREDENTIALS,
        LOGON32_PROVIDER_DEFAULT, SC_HANDLE, TOKEN_ALL_ACCESS, AdjustTokenPrivileges
    },
    System::{
        Memory::GPTR,
        Threading::{GetCurrentProcess, CreateProcessWithLogonW, DETACHED_PROCESS, LOGON_NETCREDENTIALS_ONLY, CreateProcessAsUserA, OpenProcessToken, PROCESS_INFORMATION, STARTUPINFOA, CreateProcessWithTokenW, LOGON_WITH_PROFILE},
    },
};
use clap::{Arg, App, SubCommand};


fn main() {
    let app = App::new("Token impersonation")
    .arg(Arg::with_name("DOMAIN")
        .short('d')
        .long("domain")
        .help("Domain name")
        .takes_value(true))
    .arg(Arg::with_name("USERNAME")
        .short('u')
        .long("username")
        .help("Username")
        .takes_value(true))
    .arg(Arg::with_name("PASSWORD")
        .short('p')
        .long("password")
        .help("Password")
        .takes_value(true))
    .arg(Arg::with_name("COMMAND")
        .short('c')
        .long("command")
        .help("Command to run")
        .takes_value(true));

let matches = app.get_matches();
    
let username = matches.value_of("USERNAME").expect("This field is required");
let domain = matches.value_of("DOMAIN").expect("This field is required");
let password = matches.value_of("PASSWORD").expect("This field is required");
let command = matches.value_of("COMMAND").expect("This field is required");

println!("[*] Making token with the following credentials: {}\\{}:{}", domain, username, password);

let mut username_str: String = String::new();
let mut domain_str: String = String::new();
let mut password_str: String = String::new();
let mut command_str: String = String::new();

// Prep strings for UTF-16
username_str.push_str(username);
username_str.push_str("\0");
domain_str.push_str(domain);
domain_str.push_str("\0");
password_str.push_str(password);
password_str.push_str("\0");
command_str.push_str(command);
command_str.push_str("\0");

println!("[*] Encoding into UTF-16...");

let mut username_wstr: Vec<u16> = username_str.encode_utf16().collect();
let mut domain_wstr: Vec<u16> = domain_str.encode_utf16().collect();
let mut pass_wstr: Vec<u16> = password_str.encode_utf16().collect();
let mut cmd_ptr: Vec<u16> = command_str.encode_utf16().collect();

println!("[*] Calling CreateProcessWithLogonW...");
unsafe{
    let bResult = CreateProcessWithLogonW(
        PCWSTR(username_wstr.as_mut_ptr()),
        PCWSTR(domain_wstr.as_mut_ptr()),
        PCWSTR(pass_wstr.as_mut_ptr()),
        LOGON_NETCREDENTIALS_ONLY,
        PCWSTR(null_mut()),
        PWSTR(cmd_ptr.as_mut_ptr()),
        0,
        null_mut(),
        PCWSTR(null_mut()),
        null_mut(),
        null_mut(),
    );

    println!("[*] CreateProcessWithLogon result: {:?}", bResult);
    println!("[*] GetLastError: {:?}", GetLastError());
    
    }
}
