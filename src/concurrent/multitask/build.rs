use std::process::Command;

const ASM_FILE: &str = "asm/context.S"; // assembly file
const O_FILE: &str = "asm/context.o"; // object file
const LIB_FILE: &str = "asm/libcontext.a"; // static library

fn main() {
    // -c: compile and assemble, but do not link
    // -f: specify the output file
    // PIC: Position Independent Code
    // -ggdb: include debugging information
    // -o: output file
    Command::new("cc") // cc is the C compiler
        .args(&[ASM_FILE, "-c", "-fPIC", "-ggdb", "-o"])
        .arg(O_FILE) // output file
        .status() // run the command
        .unwrap(); // panic if the command fails

    // c: create a new archive
    // r: insert the files in the archive. If the archive already exists, the files are added to the end of the archive
    // u: only insert files that are newer than the archive
    // U: only insert files that are newer than the archive or that are not already in the archive
    // s: create an archive index
    Command::new("ar") // ar is the archiver. It creates static libraries
        .args(&["cruUs", LIB_FILE, O_FILE])
        .status()
        .unwrap();

    // in build.rs, cargo run these commands

    // cargo adds the directory to the library search path
    println!("cargo:rustc-link-search=native={}", "asm");

    // cargo links the library
    println!("cargo:rustc-link-lib=static=context");

    // cargo reruns the build script if the assembly file changes
    println!("cargo:rerun-if-changed=asm/context.S");
}
