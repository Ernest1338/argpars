use std::collections::HashMap;

// ===== HELPER FUNCTIONS =====

pub fn get_args() -> Vec<String> {
    std::env::args().collect::<Vec<String>>()
}

pub fn is_value_in_a_vector_str(value: &str, vector: &[String]) -> bool {
    return vector.iter().any(|a| a == value);
}

// ===== ARGPARS TRAIT =====

pub trait Argpars {
    fn new() -> Self;
    fn no_arguments_passed(&self) -> bool;
    fn passed(&self, arg: &str) -> bool;
    fn add_argument(&mut self, argument: &str, description: &str);
    fn default_arguments_passed(&self) -> bool;
    fn wrong_arguments_passed(&self) -> bool;
    fn get_parameter_for(&self, arg: &str) -> &str;
    fn display_error_message(&self, err_type: &str, additional: &str);
    fn no_default_arguments(&mut self);
    fn display_help_screen(&self);
    fn add_help_section(&mut self, section: &str, content: &str);
    fn pars(&self) -> i32;
    fn lookup_update(&mut self);
}

// ===== ARGSOBJ STRUCT =====

pub struct ArgsObj {
    pub arguments_passed_args: std::env::Args,
    pub arguments_passed: Vec<String>,
    pub number_of_arguments: u32,
    pub arguments: Vec<String>,
    pub default_arguments: bool,
    pub help_usage: String,
    pub help_name: String,
    pub help_description: String,
    pub help_version: String,
    pub arg_desc_vec: Vec<String>,
    pub help_sections: Vec<String>,
    pub help_sections_content: Vec<String>,
    pub passed_arguments_lookup: HashMap<String, bool>,
    pub parameters_lookup: HashMap<String, String>,
    pub last_param_ok: bool,
}

// ===== IMPLEMENTATION OF ARGPARS FOR ARGSOBJ =====

impl Argpars for ArgsObj {
    // ArgsObj constructor
    fn new() -> ArgsObj {
        return ArgsObj {
            arguments_passed_args: std::env::args(),
            arguments_passed: get_args(),
            number_of_arguments: std::env::args().len() as u32,
            arguments: vec!["--help".to_string(), "--version".to_string()],
            default_arguments: true,
            help_usage: format!("Usage: {} [OPTION]...\n", get_args()[0]),
            help_name: "Default name".to_string(),
            help_description: "Default description".to_string(),
            help_version: "Default version".to_string(),
            arg_desc_vec: vec![
                "--help".to_string(),
                "\tdisplay this help and exit".to_string(),
                "--version".to_string(),
                "output version information and exit".to_string(),
            ],
            help_sections: Vec::new(),
            help_sections_content: Vec::new(),
            passed_arguments_lookup: HashMap::from([
                ("--help".to_string(), false),
                ("--version".to_string(), false),
            ]),
            parameters_lookup: HashMap::from([
                ("--help".to_string(), "".to_string()),
                ("--version".to_string(), "".to_string()),
            ]),
            last_param_ok: false,
        };
    }

    // Function which updates lookup HashMaps such as passed_arguments_lookup or parameters_lookup
    fn lookup_update(&mut self) {
        for arg in &self.arguments {
            if is_value_in_a_vector_str(arg, &self.arguments_passed) {
                *self.passed_arguments_lookup.get_mut(&*arg).unwrap() = true;
                *self.parameters_lookup.get_mut(&*arg).unwrap() =
                    self.get_parameter_for(arg).to_string();
            }
        }
    }

    // Function which, when called, disables default arguments
    fn no_default_arguments(&mut self) {
        for _ in 0..2 {
            self.arguments.remove(0);
        }
        for _ in 0..4 {
            self.arg_desc_vec.remove(0);
        }
        self.passed_arguments_lookup.remove_entry("--help");
        self.passed_arguments_lookup.remove_entry("--version");
        self.parameters_lookup.remove_entry("--help");
        self.parameters_lookup.remove_entry("--version");
        self.default_arguments = false;
    }

    // Function returning if no arguments were passed
    fn no_arguments_passed(&self) -> bool {
        self.number_of_arguments == 1
    }

    // Function which checks if an arguments was passed
    fn passed(&self, arg: &str) -> bool {
        is_value_in_a_vector_str(arg, &self.arguments_passed)
    }

    // Function used to add an argument into the app
    fn add_argument(&mut self, argument: &str, description: &str) {
        self.arguments.push(argument.to_string());
        self.arg_desc_vec.push(argument.to_string());
        self.arg_desc_vec.push(description.to_string());
        self.passed_arguments_lookup
            .insert(argument.to_string(), false);
        self.parameters_lookup
            .insert(argument.to_string(), "".to_string());
        self.lookup_update();
    }

    // Function returning if default arguments were passed
    fn default_arguments_passed(&self) -> bool {
        self.passed("--help") || self.passed("--version")
    }

    // Function returning if wrong (non existent) arguments / parameters were passed
    fn wrong_arguments_passed(&self) -> bool {
        let mut loop_end: usize = self.number_of_arguments as usize;
        if self.last_param_ok {
            loop_end -= 1;
        }
        for i in 1..loop_end {
            if self.arguments_passed[i as usize].starts_with('-') {
                if !is_value_in_a_vector_str(&self.arguments_passed[i as usize], &self.arguments) {
                    return true;
                }
            } else if !is_value_in_a_vector_str(
                &self.arguments_passed[(i - 1) as usize],
                &self.arguments,
            ) {
                return true;
            }
        }
        false
    }

    // Function used to retrive passed parameter to an argument
    fn get_parameter_for(&self, arg: &str) -> &str {
        let index_of_argument: usize = self.arguments_passed.iter().position(|r| r == arg).unwrap();
        let index_of_parameter: usize = index_of_argument + 1;
        if index_of_parameter < self.arguments_passed.len()
            && !is_value_in_a_vector_str(
                &self.arguments_passed[index_of_parameter],
                &self.arguments,
            )
        {
            return &self.arguments_passed[index_of_parameter];
        }

        ""
    }

    // Function used to display error messages
    fn display_error_message(&self, err_type: &str, additional: &str) {
        if err_type == "no_such_option" {
            eprintln!("ERROR: No such option: \'{}\'", additional);
            eprintln!(
                "Try: \'{} --help\' for more information.",
                self.arguments_passed[0]
            );
        }
    }

    // Function used to display the help screen
    fn display_help_screen(&self) {
        println!("{}", self.help_usage);
        println!("Name: {}", self.help_name);
        println!("Description: {}", self.help_description);
        println!("Version: {}\n", self.help_version);
        println!("Possible options:");
        for arg in &self.arguments {
            if is_value_in_a_vector_str(arg, &self.arg_desc_vec) {
                let desc_index: usize =
                    self.arg_desc_vec.iter().position(|a| a == arg).unwrap() + 1;
                println!("\t{}\t{}", arg, self.arg_desc_vec[desc_index]);
            } else {
                println!("\t{}", arg);
            }
        }
        if !self.help_sections.is_empty() {
            println!();
            for section in &self.help_sections {
                println!("{}", section);
                if is_value_in_a_vector_str(section, &self.help_sections_content) {
                    let content_index: usize = self
                        .help_sections_content
                        .iter()
                        .position(|a| a == section)
                        .unwrap()
                        + 1;
                    println!("{}", self.help_sections_content[content_index]);
                }
            }
        }
    }

    // Function used to add a section into the help screen
    fn add_help_section(&mut self, section: &str, content: &str) {
        self.help_sections.push(section.to_string());
        self.help_sections_content.push(section.to_string());
        self.help_sections_content.push(content.to_string());
    }

    // Main Argpars parser
    fn pars(&self) -> i32 {
        if self.no_arguments_passed() {
            // // Displaying help screen if no arguments were passed (disabled by default):
            // self.display_help_screen();
        } else {
            let mut loop_end: usize = self.number_of_arguments as usize;
            if self.last_param_ok {
                loop_end -= 1;
            }
            for i in 1..loop_end {
                // If there is a '-' character at the beginning and it is not an known argument, throw an error
                if self.arguments_passed[i as usize].starts_with('-') {
                    if !is_value_in_a_vector_str(
                        &self.arguments_passed[i as usize],
                        &self.arguments,
                    ) {
                        self.display_error_message(
                            "no_such_option",
                            &self.arguments_passed[i as usize],
                        );
                        return 1;
                    }
                }
                // If there is no '-' character at the beginning and the previous argument is now a known one, throw an error
                else if !is_value_in_a_vector_str(
                    &self.arguments_passed[(i - 1) as usize],
                    &self.arguments,
                ) {
                    self.display_error_message(
                        "no_such_option",
                        &self.arguments_passed[i as usize],
                    );
                    return 1;
                }
            }
            if self.default_arguments {
                if self.passed("--help") {
                    self.display_help_screen();
                }
                if self.passed("--version") {
                    println!("{} version: {}", self.help_name, self.help_version);
                }
            }
        }
        0
    }
}