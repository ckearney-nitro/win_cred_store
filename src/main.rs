use windows::{
    core::{Error, HRESULT, PCWSTR},
    Win32::{
        Foundation::GetLastError,
        Security::Credentials::{CredDeleteW, CRED_TYPE_GENERIC},
    },
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        help();
        return;
    }

    let command = args[1].as_str();

    if command.eq_ignore_ascii_case("--remove") || command.eq_ignore_ascii_case("-r") {
        remove(&args, 1)
    } else if command.eq_ignore_ascii_case("--help") || command.eq_ignore_ascii_case("-h") {
        help();
    } else {
        println!("Unknown arg(s)");
        help();
    }
}

fn help() {
    println!("******************\n\nWin Cred Store Help\n");

    println!("--remove/-r <NAME>");
    println!("\tRemove blobs from windows cred store");
}

fn remove(args: &Vec<String>, index: usize) {
    let target_name = &args[index + 1];

    println!("Attempint to remove {} from cert store", target_name);

    let mut utf16: Vec<u16> = target_name.encode_utf16().collect();
    utf16.push(0);

    let pcwstr = PCWSTR(utf16.as_ptr());

    unsafe {
        if CredDeleteW(pcwstr, CRED_TYPE_GENERIC.0, 0).as_bool() == true {
            println!("Removed {} from cred store", target_name);
        } else {
            println!("Failed to remove {} from cred store", target_name);
            println!("{:?}", Error::fast_error(HRESULT::from(GetLastError())));
        }
    }
}
