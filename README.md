# Anydesk Waiting Time Bypass

## Overview

Anydesk, a popular remote desktop application, imposes a wait time of 100 seconds, 200 seconds, or more for free users before each connection. This program, "Anydesk Waiting Time Bypass," is designed to bypass, reset, or remove this wait time. It achieves this by managing Anydesk processes and directories, potentially resetting application data and cache. Please note that using this tool may change the Anydesk ID assigned to your system.

## Disclaimer

**Use this tool at your own risk and responsibility.** This is an educational project, and the developers are not responsible for any consequences arising from its use. It has been tested and designed to work with the direct run mode of Anydesk, not the installed version.


## Prerequisites

- Rust (latest stable version)

## Installation

1. **Clone the Repository:**
   ```sh
   git clone https://github.com/asman1337/anydesk-waiting-time-bypass.git
   cd anydesk-waiting-time-bypass
   ```

2. **Build the Project:**
   ```sh
   cargo build --release
   ```

3. **Run the Program:**
   ```sh
   ./target/release/anydesk-waiting-time-bypass
   ```

## Usage

1. **Launch the Program:**
   ```sh
   ./target/release/anydesk-waiting-time-bypass
   ```

2. **Confirmation Prompt:**
   The program will ask for confirmation before killing the Anydesk process:
   ```
   This will kill the Anydesk process. Are you willing to continue? (y/N)
   ```

3. **Operation Success:**
   If confirmed, the program will terminate the Anydesk process and optionally restart it based on user input.

4. **Cancellation:**
   If the user cancels the operation, the program will exit without making any changes.

## Example Output

```
****************************************
*      Anydesk Waiting Time Resetter   *
*            by Asman Mirza            *
****************************************

This will kill the Anydesk process. Are you willing to continue? (y/N) y
Operation was successful.

Do you want to start the Anydesk process? (y/N) y
Anydesk process started.
```

## Important Notes

- **Designed for Direct Run Mode:** This tool is designed to work with the direct run mode of Anydesk, not the installed version.
- **Potential Side Effects:** Using this tool may reset the application data and cache, and change the Anydesk ID assigned to your system.
- **Use at Your Own Risk:** This is an educational project, and the developers are not responsible for any consequences arising from its use.

## License

This project is licensed under the MIT License.

## Acknowledgements

Developed by Asman Mirza.

---

This README provides a comprehensive overview of the project, its features, and usage instructions, while also clearly stating the potential risks and the educational nature of the tool.