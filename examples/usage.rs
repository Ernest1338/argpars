use argpars::*;

fn print_stuff() {
    println!("stuff");
}

fn print_param(to_print: &str) {
    println!("{}", to_print);
}

fn main() {
    // Creating ArgsObj object
    let mut args: ArgsObj = Argpars::new();
    // // To disable default arguments (--help and --version) uncomment this line:
    //args.no_default_arguments();

    // Setting basic info about the app
    args.help_usage = format!("Usage: {} [OPTION]... [TEST]\n", args.arguments_passed[0]);
    args.help_name = "Test App".to_string();
    args.help_description = "This is a test description".to_string();
    args.help_version = "v1.0".to_string();

    // Adding sections into the help screen
    args.add_help_section("TEST SECTION:", "\tthis is a test section!\n");
    args.add_help_section(
        "SECOND TEST SECTION:",
        "\tthis is another test section!\n\tWith multiple lines!",
    );

    // Adding arguments into the app
    args.add_argument("--print-stuff", "display \"stuff\"");
    args.add_argument("--print-param", "display whatever you pass as an parameter");

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
            print_stuff();
        }
        if args.passed("--print-param") {
            print_param(args.get_parameter_for("--print-param"));
        }
    }

    // Executing Argpars parser and exiting from the app with a return value
    std::process::exit(args.pars());
}
