use std::collections::HashMap;
use indexmap::map::IndexMap;
use crate::command_line::parameter::{MandatoryParameter, OptionalParameter};

pub mod parameter;

type Name = String;

/// A command line specification.
pub struct CommandLine<'a> {
    /// An "action" defines a "sub command line".
    /// Ex: `git add ...`, `git commit ...`
    ///     "add" and "commit" are "actions".
    /// Please note that an "action" must follow the executable name or another "action".
    actions: HashMap<Name, &'a CommandLine<'a>>,
    /// Optional parameters with an "explicit" value - that is "options".
    /// Ex: `--input=<value>` or `-i=<value>`.
    option: HashMap<Name, OptionalParameter>,
    /// Optional parameters with an "implicit" value (true/false) - that is "flags".
    /// Ex: `--verbose` or `-v`.
    flag: HashMap<Name, OptionalParameter>,
    /// Mandatory parameters that appear at the end of the command line.
    /// Please note that the declaration order for these parameters is important.
    parameter: IndexMap<Name, MandatoryParameter>
}

impl<'a> CommandLine<'_> {
    pub fn new() -> CommandLine<'a> {
        return CommandLine {
            actions: HashMap::new(),
            option: HashMap::new(),
            flag: HashMap::new(),
            parameter: IndexMap::new()
        }
    }

    pub fn add_option(&mut self,
                      name: String,
                      long: Option<String>,
                      short: Option<char>,
                      max_occurrence:Option<u8>) {
        if self.option.contains_key(name.as_str()) {
            panic!("An option with the name {} already exists!", name)
        }
        self.option.insert(name, OptionalParameter::new(long, short, max_occurrence));
    }

    pub fn add_flag(&mut self,
                    name: String,
                    long: Option<String>,
                    short: Option<char>,
                    max_occurrence:Option<u8>) {
        if self.option.contains_key(name.as_str()) {
            panic!("A flag with the name {} already exists!", name)
        }
        self.flag.insert(name, OptionalParameter::new(long, short, max_occurrence));
    }

    pub fn add_parameter(&mut self,
                         name: String,
                         max_occurrence: Option<u8>) {
        if self.parameter.contains_key(name.as_str()) {
            panic!("A parameter with the name {} already exists!", name)
        }
        let last_entry = self.parameter.last();
        if last_entry.is_some() {
            let last_param = last_entry.unwrap().1;
            if last_param.get_max_occurrence() > 1 {
                panic!("Cannot add the parameter whose name is \"{}\". The previously declared \
                parameter (whose name is \"{}\") is associated with a maximum number of occurrences \
                greater than 1 ({}). Only the last parameter can be associated with a maximum number \
                of occurrences greater than 1.",
                       name, last_entry.unwrap().0, last_param.get_max_occurrence())
            }
        }
        self.parameter.insert(name, MandatoryParameter::new(max_occurrence));
    }

    fn add_action(&mut self,
                  name: String,
                  command_line: Option<CommandLine>) -> CommandLine {
        if self.actions.contains_key(name.as_str()) {
            panic!("An action with the name {} already exists!", name)
        }
        let cl: CommandLine;
        if command_line.is_some() {
            cl = command_line.unwrap();
        } else {
            cl = CommandLine::new();
        }

        self.actions.insert(name, &cl);
        return cl;
    }
}