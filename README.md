# argpars

Dependency-less, simple yet functional Command Line Argument Parser

# Usage

Basic usage (checkout the examples/usage.rs file for more information)

```rust
use argpars::*;

fn main() {
    let mut args: ArgsObj = Argpars::new();

    // Setting basic info about the app
    args.help_usage = format!("Usage: {} [OPTION]... [TEST]\n", args.arguments_passed[0]);
    args.help_name = "Test App".to_string();
    args.help_description = "This is a test description".to_string();
    args.help_version = "v1.0".to_string();

    // Adding arguments into the app
    args.add_argument("--print-stuff", "display \"stuff\"");

    // This is how you execute something when no arguments were passed
    if args.no_arguments_passed() {
        args.display_help_screen();
    }
    // This is how you ignore other arguments when the default (help, version) or wrong ones were passed
    else if args.default_arguments_passed() || args.wrong_arguments_passed() {
    }
    // Here you handle the rest of the arguments
    else {
        if args.passed("--print-stuff") {
            println!("stuff");
        }
    }

    // Executing Argpars parser and exiting from the app with a return value
    std::process::exit(args.pars());
}
```

# LICENSE

This project is distributed under MIT license.
