// Program to test rusts's memory when its used as a "forkbomb" (on my machine)
// DANGER: This software is experimental and intended for educational use only. Running it could cause your system to crash or require a hard reboot.
use std::env;
use std::process::Command;

fn main() {
    // Check if the bypass argument is provided
    let args: Vec<String> = env::args().collect();
    let bypass_warning = args.get(1).map_or(false, |arg| arg == "--bypass");

    // Print the warning and license if bypass not set
    if !bypass_warning {
        println!("Warning: This program will consume all available memory on your machine and may cause it to crash");
        println!("Do not run this program on your machine");
        println!("This program is only for educational purposes");
        println!("This program will create an unstoppable loop of processes, which will eventually require a hard reboot to recover.");
        println!("If you do not fully understand the implications of this program, please close it now");
        println!("If you want to run this program without this warning, use the --bypass argument");
        println!();

        // License information
        println!("LICENSE: MIT (c) 2024 Laura E.");
        println!("https://opensource.org/licenses/MIT");
        println!();

        // Wait for input to start the program
        println!("Press enter to start the program");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
    }

    // Get the path of the current executable
    let current_exe = env::current_exe().expect("Failed to get current executable path");

    // Infinite loop to continuously spawn new instances of itself
    loop {
        Command::new(&current_exe)
            .arg("--bypass") // Pass the bypass argument to each spawned instance
            .spawn()
            .expect("Failed to spawn process");
    }
}


// Warning: This program will consume all available memory on your machine and may cause it to crash
// Do not run this program on your machine
// This program is only for educational purposes

// LICENSE: MIT (c) 2024 Laura E. 
// https://opensource.org/licenses/MIT

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. 
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, 
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
