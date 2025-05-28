# OS and Version Checker:
This Rust program attempts to detect the operating system and its version details by executing native system commands. It provides specific information for Linux, macOS, and Windows.

# Features:
Linux Detection: Uses uname -s -r to identify Linux and display its kernel version.
macOS Detection: Uses uname -s -r for initial detection and sw_vers for detailed version information.
Windows Detection: Uses wmic os get Caption,Version,BuildNumber /value to retrieve comprehensive OS details.
Command-Based: Determines OS by executing system commands, providing a potentially more detailed or alternative approach compared to compile-time or std::env::consts::OS checks.

# How it Works:
The program first attempts to run uname -s -r, which is a common command on Unix-like systems (Linux, macOS, etc.) to get the kernel name and release.

If uname indicates "Linux", it prints the detected OS and version.
If uname indicates "Darwin" (macOS), it then executes sw_vers to get more detailed macOS version information.
If uname fails or indicates an unknown Unix-like OS, the program proceeds to check for Windows.
For Windows detection, the program executes wmic os get Caption,Version,BuildNumber /value to retrieve specific OS details like the caption (e.g., "Microsoft Windows 10 Pro"), version number, and build number.

# Getting Started:

1. Prerequisites

Rust and Cargo: You need to have Rust and its package manager, Cargo, installed on your system. If you don't have them, you can install them by following the instructions on the official Rust website.

2. Building and Running

Clone the repository (or save the file):

2.1. If you're using Git, clone your project:

Bash

git clone https://github.com/shadyxur/os-check.git 

cd os-check

Alternatively, if you just have the os-check.rs file, navigate to its directory.

2.2. Compile the code:

Open your terminal or command prompt in the directory containing os-check.rs and run:

Bash

cargo build --release

The --release flag compiles the code with optimizations, resulting in a faster executable.

Run the executable:
After successful compilation, you can run the program. The executable will be located in the target/release/ directory.

2.3. On Linux/macOS:

Bash

./target/release/os-check

2.4. On Windows (in Command Prompt or PowerShell):

Bash

.\target\release\os-check.exe

The program will then output the detected OS and its version information.

# Contributing:
Feel free to open issues or submit pull requests if you have suggestions for improvements or bug fixes.

# License
This project is licensed under the MIT License.
