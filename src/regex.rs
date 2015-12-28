use libc::{c_int, c_uint, c_void};
use std::{error, fmt, str, ptr};

use super::{Region, Encoding, Options, Syntax, ENCODING_UTF8, SYNTAX_RUBY, OPTION_NONE};

type OnigRegex = *const c_void;

#[link(name="onig")]
extern {
    fn onig_error_code_to_str(err_buff: *mut u8, err_code: c_int, ...) -> c_int;

    fn onig_new(
        reg: *mut OnigRegex,
        pattern: *const u8,
        pattern_end: *const u8,
        option: c_uint,
        enc: *const Encoding,
        syntax: *const Syntax,
        err_info: *mut OnigErrorInfo
    ) -> c_int;

    fn onig_search(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        start: *const u8,
        range: *const u8,
        region: *const Region,
        option: c_uint
    ) -> c_int;

    fn onig_match(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        at: *const u8,
        region: *const Region,
        option: c_uint
    ) -> c_int;

    fn onig_number_of_names(reg: OnigRegex) -> c_int;
    fn onig_number_of_captures(reg: OnigRegex) -> c_int;
    fn onig_number_of_capture_histories(reg: OnigRegex) -> c_int;

    fn onig_free(reg: OnigRegex);
}

#[repr(C)]
#[derive(Debug)]
struct OnigErrorInfo {
    enc: *const c_void, // TODO: change type to Encoding
    par: *const u8,
    par_end: *const u8
}

/// An error that occurred during parsing, compiling or evaluating
/// a regular expression.
pub struct Error {
    error: c_int,
    description: String,
}

impl Error {
    fn new(error: c_int, info: Option<OnigErrorInfo>) -> Error {
        let mut err_buff = &mut [0 as u8; 90];
        let len = unsafe {
            match info {
                Some(ref error_info) =>
                    onig_error_code_to_str(
                        err_buff.as_mut_ptr(),
                        error,
                        error_info as *const OnigErrorInfo
                    ),
                None => onig_error_code_to_str(err_buff.as_mut_ptr(), error)
            }
        };
        let description = str::from_utf8(&err_buff[..len as usize]).unwrap();
        Error { error: error, description: description.to_owned() }
    }
}

impl Error {
    /// Return Oniguruma engine error code.
    pub fn code(&self) -> isize {
        self.error as isize
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}, {})", self.error, self.description)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oniguruma error: {}", self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct RegexConfig<'a> {
    pub options: Options,
    pub syntax: &'a Syntax
    // TODO: add encoding here
    // TODO: should we add case fold flag and other compile time info
    // (see: `onig_new_deluxe`)?
}

impl Default for RegexConfig<'static> {
    fn default() -> RegexConfig<'static> {
        RegexConfig {
            options: OPTION_NONE,
            syntax: SYNTAX_RUBY
        }
    }
}

/// A compiled Oniguruma regular expression.
#[derive(Debug)]
pub struct Regex {
    raw: OnigRegex
}

impl Regex {
    /// Compiles a regular expression with default options. Default syntax is
    /// `SYNTAX_RUBY`.
    ///
    /// Once compiled, it can be used repeatedly to search in a string. If an
    /// invalid expression is given, then an error is returned.
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        Regex::new_with_config(pattern, RegexConfig::default())
    }

    pub fn new_with_config<'a>(pattern: &str, config: RegexConfig<'a>) -> Result<Regex, Error> {
        // Convert the rust types to those required for the call to
        // `onig_new`.
        let pattern_bytes = pattern.as_bytes();
        let (start, end) = (
            pattern_bytes.as_ptr(),
            pattern_bytes[pattern_bytes.len()..].as_ptr()
        );
        let mut reg: OnigRegex = ptr::null();
        let reg_ptr = &mut reg as *mut OnigRegex;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = OnigErrorInfo {
            enc: ptr::null(),
            par: ptr::null(),
            par_end: ptr::null()
        };

        let err = unsafe {
            onig_new(
                reg_ptr,
                start,
                end,
                config.options.bits(),
                ENCODING_UTF8,
                config.syntax,
                &mut error)
        };

        if err == 0 {
            Ok(Regex{ raw: reg })
        } else {
            Err(Error::new(err, Some(error)))
        }
    }

    /// Search pattern in string and store search result into region object.
    ///
    /// Returns match position offset if pattern is found, otherwise return
    /// `None`. You also can use search time options: `OPTION_NOTBOL` and
    /// `OPTION_NOTEOL`.
    pub fn search_with_region(&self,
                              text: &str,
                              region: &mut Region,
                              options: Options)
                              -> Result<Option<usize>, Error> {
        let text_bytes = text.as_bytes();
        let (start, end) = (
            text_bytes.as_ptr(),
            text_bytes[text_bytes.len()..].as_ptr()
        );

        let r = unsafe {
            onig_search(
                self.raw,
                start,
                end,
                start,
                end,
                region,
                options.bits()
            )
        };

        if r >= 0 {
            Ok(Some(r as usize))
        } else if r == -1 {
            Ok(None)
        } else {
            Err(Error::new(r, None))
        }
    }

    /// Match string and store search result into region object.
    ///
    /// Returns match length if pattern is found, otherwise return `None`.
    /// You also can use search time options: `OPTION_NOTBOL` and
    /// `OPTION_NOTEOL`.
    pub fn match_with_region(&self,
                             text: &str,
                             region: &mut Region,
                             options: Options)
                             -> Result<Option<usize>, Error> {
        let text_bytes = text.as_bytes();
        let (start, end) = (
            text_bytes.as_ptr(),
            text_bytes[text_bytes.len()..].as_ptr()
        );

        let r = unsafe {
            onig_match(
                self.raw,
                start,
                end,
                start,
                region,
                options.bits()
            )
        };

        if r >= 0 {
            Ok(Some(r as usize))
        } else if r == -1 {
            Ok(None)
        } else {
            Err(Error::new(r, None))
        }
    }

    /// Returns true if and only if the regex matches the string given.
    ///
    /// # Panics
    ///
    /// This method may panic in the case of memory overflow during execution or
    /// other internal errors of Oniguruma engine.
    pub fn is_match(&self, text: &str) -> bool {
        let mut region = Region::new();
        self.match_with_region(text, &mut region, OPTION_NONE)
            .unwrap()
            .map(|r| r == text.len())
            .unwrap_or(false)
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// Note that this should only be used if you want to discover the position
    /// of the match. Testing the existence of a match is faster if you use
    /// `is_match`.
    ///
    /// # Panics
    ///
    /// This method may panic in the case of memory overflow during execution or
    /// other internal errors of Oniguruma engine.
    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        let mut region = Region::new();
        self.search_with_region(text, &mut region, OPTION_NONE)
            .unwrap()
            .map(|_| region.pos(0))
            .unwrap_or(None)
    }

    pub fn captures_len(&self) -> usize {
        unsafe {
            onig_number_of_captures(self.raw) as usize
        }
    }

    pub fn capture_histories_len(&self) -> usize {
        unsafe {
            onig_number_of_capture_histories(self.raw) as usize
        }
    }

    pub fn names_len(&self) -> usize {
        unsafe {
            onig_number_of_names(self.raw) as usize
        }
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe {
            onig_free(self.raw);
        }
    }
}

