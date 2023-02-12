// use std::collections::HashMap;
use command_line as cl;



fn main() {

    // We define a command line without switches.

    // let common_optional_parameters: &[cl::OptionalParameter] = &[
    //     cl::OptionalParameter {
    //         name: String::from("o:input-format"),
    //         long: Some(String::from("input-format")),
    //         short: None,
    //         max_occurrence: 1,
    //         validator: None
    //     },
    //     cl::OptionalParameter {
    //         name: String::from("o:output-format"),
    //         long: Some(String::from("output-format")),
    //         short: None,
    //         max_occurrence: 1,
    //         validator: None
    //     },
    //     cl::OptionalParameter {
    //         name: String::from("o:debug-level"),
    //         long: Some(String::from("debug-level")),
    //         short: Some(String::from("d")),
    //         max_occurrence: 1,
    //         validator: None
    //     }
    // ];
    //
    // let common_flags: &[cl::Flag] = &[
    //     cl::Flag {
    //         name: String::from("f:verbose"),
    //         long: Some(String::from("verbose")),
    //         short: Some(String::from("v")),
    //         max_occurrence: 3
    //     }
    // ];
    //
    // let common_mandatory_parameters: &[cl::Parameter] = &[
    //     cl::Parameter {
    //         name: String::from("p:output"),
    //         max_occurrence: 1
    //     },
    //     cl::Parameter {
    //         name: String::from("p:inputs"),
    //         max_occurrence: 3
    //     }
    // ];
    //
    // let command_line = cl::Specification {
    //     optional_parameters: Some(common_optional_parameters),
    //     flags: Some(common_flags),
    //     parameters: Some(common_mandatory_parameters),
    //     switchers: None
    // };


}
