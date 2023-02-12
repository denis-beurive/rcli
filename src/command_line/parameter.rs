
/// An optional parameter with an "explicit" value is called an "option". Ex: "--input=<value>" or
/// "-i=<value>"
///
/// An optional parameter without "explicit" value has an "implicit" value. The value is a boolean
/// that indicates whether the parameter appears on the command line or not). Optional parameters
/// with "implicit" values are called "flags". Ex: "--verbose" or "-v".
pub struct OptionalParameter {
    /// A string that represents the "long identifier" for this optional parameter.
    /// For example, is the long option appears as "--input" on the command line, then the value
    /// of this field is "input".
    long: Option<String>,
    /// A string that represents the "short identifier" for this optional parameter.
    /// For example, if the short option appears as "-i" on the command line, then the value
    /// of this field is "i".
    short: Option<char>,
    /// The maximum number of times that this optional parameter can appear within the command line.
    /// This value, if specified, must be greater or equal to 1.
    max_occurrence: u8
}

pub struct MandatoryParameter {
    /// The maximum number of times that this parameter can appear within the command line.
    /// If the parameter is the last of the command line, then it can appear more than once.
    /// Otherwise, it appears once.
    max_occurrence: u8
}

fn get_max_occurrence(max_occurrence: Option<u8>) -> u8 {
    if max_occurrence.is_none() { return 1; }
    if 0 == max_occurrence.unwrap() { return 1 }
    return max_occurrence.unwrap();
}

/// Check the (long and short) identifiers of an optional parameter.
// fn check_identifiers(long: &Option<String>, short: &Option<char>) -> () {
//     if long.is_none() && short.is_none() {
//         panic!("Long and short optional parameters identifiers cannot be undefined simultaneously")
//     }
//     if long.is_none() { return; }
//     if 0 == long.as_ref().unwrap().len() { panic!("A long parameter identifier cannot not be an empty string"); }
// }

impl OptionalParameter {
    pub fn new(long: Option<String>, short: Option<char>, max_occurrence:Option<u8>) -> OptionalParameter {

        if long.is_none() && short.is_none() {
            panic!("Long and short optional parameters identifiers cannot be undefined simultaneously")
        }
        if long.is_some() {
            if 0 == long.as_ref().unwrap().len() { panic!("A long parameter identifier cannot not be an empty string"); }
        }

        return OptionalParameter { long, short, max_occurrence: get_max_occurrence(max_occurrence) };
    }
}

impl MandatoryParameter {
    pub fn new(max_occurrence: Option<u8>) -> MandatoryParameter {
        return MandatoryParameter { max_occurrence: get_max_occurrence(max_occurrence) };
    }

    pub fn get_max_occurrence(&self) -> u8 {
        return self.max_occurrence;
    }
}



