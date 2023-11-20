use clap::Parser;
use pe_parser::coff::MachineTypes;
use pe_parser::pe;
use std::fs;
use std::env::consts::ARCH;

mod dlls;
mod loader;
mod types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    exec: String,
}

fn main() {
    let args = Args::parse();
    let binary = fs::read(args.exec).expect("Failed to read specified executable");

    let pe = pe::parse_portable_executable(binary.as_slice())
        .expect("Supplied executable is not a valid PE!");

    let machine = pe.coff.get_machine_type().expect("Unknown machine type!");

    // There are more possible architectures, only support
    // most common for now x86/x86_64 and arm/aarch64
    let local_machine = match ARCH {
        "x86" => MachineTypes::I386,
        "x86_64" => MachineTypes::AMD64,
        "arm" => MachineTypes::ARM,
        "aarch64" => MachineTypes::ARM64,
        _ => MachineTypes::Unknown,
    };

    // In the future this could be expanded to handle simple
    // incompatibilities like x86 on x86_64, or more complex ones
    // like ARM on x86 through JIT. For now, only allow native code.
    if machine != local_machine {
        panic!("Executable architecture does not match host architecture!");
    }
}
