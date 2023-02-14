
/// Optional parameter
///
/// An optional parameter with an "explicit" value is called an "option". Ex: `--input=<value>` or
/// `-i=<value>`
///
/// An optional parameter without (explicit) value is called a "flag". Ex: `--verbose` or `-v`.
/// A "flag" has an "implicit" value. The value is a boolean that indicates whether the parameter
/// appears on the command line or not).  Ex: `--verbose` or `-v`.
///
/// An optional parameter may be associated with a "long identifier" (`--input=<value>` or
/// `--verbose`), a short identifier (`-i=<input>` or `-v`), or both.
pub struct OptionalParameter {
    /// A string that represents the "long identifier" for this optional parameter.
    /// For example, if the long option appears as `--input` on the command line, then the value
    /// of this field is "input".
    long: Option<String>,
    /// A string that represents the "short identifier" for this optional parameter.
    /// For example, if the short option appears as `-i` on the command line, then the value
    /// of this field is "i".
    short: Option<char>,
    /// The maximum number of times that this optional parameter can appear within the command line.
    /// This value, if specified, must be greater or equal to 1.
    max_occurrence: u8
}

/// A mandatory parameter appears at the end of the command line, after all optional parameters
/// Mandatory parameters are identified by their positions on the command line.
pub struct MandatoryParameter {
    /// The maximum number of times that this parameter can appear within the command line.
    /// If the parameter is the last of the command line, then it can appear more than once.
    /// Otherwise, it appears once.
    ///
    /// If the value of this field is None, it means that this mandatory parameter can appear up
    /// to 255 times on the command line (the maximum value for an `u8`).
    max_occurrence: Option<u8>
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
    fn get_max_occurrence(in_max_occurrence: Option<u8>) -> u8 {
        if in_max_occurrence.is_none() { return 1; }
        let max_occurrence = in_max_occurrence.unwrap();
        if 0 == max_occurrence { return 1 }
        return max_occurrence;
    }

    pub fn new(in_long: Option<String>, in_short: Option<char>, in_max_occurrence:Option<u8>) -> OptionalParameter {
        if in_long.is_none() && in_short.is_none() {
            panic!("Long and short optional parameters identifiers cannot be undefined simultaneously")
        }
        if in_long.is_some() {
            if 0 == in_long.as_ref().unwrap().len() { panic!("A long parameter identifier cannot not be an empty string"); }
        }
        return OptionalParameter { long: in_long, short: in_short, max_occurrence: get_max_occurrence(in_max_occurrence) };
    }
}

impl MandatoryParameter {
    pub fn new(in_max_occurrence: Option<u8>) -> MandatoryParameter {
        if 0 == in_max_occurrence { panic!("A mandatory parameter must appear at least one time on the command line") }
        return MandatoryParameter { max_occurrence: in_max_occurrence };
    }

    pub fn get_max_occurrence(&self) -> Option<u8> {
        return self.max_occurrence;
    }
}



