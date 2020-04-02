# rust-password-generator
Command line password generator written in Rust

## Running the program

Precompiled binaries available here:

Windows 64-bit: [rust-password-generator.exe](https://github.com/MyYogurt/rust-password-generator/blob/master/binaries/rust-password-generator.exe?raw=true "Windows 64-bit executable") (Tested on Windows 10 64-bit)

Linux 64-bit: [rust-password-generator](https://github.com/MyYogurt/rust-password-generator/blob/master/binaries/rust-password-generator?raw=true "Linux 64-bit executable") (Tested on Ubuntu 18.04 and Fedora 31)

All other platforms will need to compile on their own machines. This can be easily done by executing:

`cargo build --release`

The program can then be run by executing:

On Windows:

`target\release\rust-password-generator.exe`

On Linux:

`./target/release/rust-password-generator`


## Example Usage:

For help, execute the program with the -h flag

Windows:

To generate a password with no numbers and a length of 15 characters:

`rust-password-generator.exe -l 15 -n false`


Linux:

To generate a password with special characters and a length of 5:

`./rust-password-generator.exe -l 5 -s`
