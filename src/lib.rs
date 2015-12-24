extern crate libc;

#[macro_use]
extern crate bitflags;

mod ll;

#[cfg(test)]
mod test;

use std::{error, fmt, str, ptr};

pub static SYNTAX_ASIS: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxASIS;
pub static SYNTAX_POSIX_BASIC: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxPosixBasic;
pub static SYNTAX_POSIX_EXTENDED: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxPosixExtended;
pub static SYNTAX_EMACS: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxEmacs;
pub static SYNTAX_GREP: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxGrep;
pub static SYNTAX_GNU_REGEX: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxGnuRegex;
pub static SYNTAX_JAVA: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxJava;
pub static SYNTAX_PERL: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxPerl;
pub static SYNTAX_PERL_NG: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxPerl_NG;
pub static SYNTAX_RUBY: *const ll::OnigSyntaxTypeStruct =  &ll::OnigSyntaxRuby;

pub static ENCODING_UTF8: *const ll::OnigEncodingType = &ll::OnigEncodingUTF8;

bitflags!{
    flags OptionType: ll::OnigOptionTypeBits {
        const OPTION_NONE = 0,
        const OPTION_IGNORECASE = 1,
        const OPTION_EXTEND = 2,
        const OPTION_MULTILINE = 4,
        const OPTION_SINGLELINE = 8,
        const OPTION_FIND_LONGEST = 16,
        const OPTION_FIND_NOT_EMPTY = 32,
        const OPTION_NEGATE_SINGLELINE = 64,
        const OPTION_DONT_CAPTURE_GROUP = 128,
        const OPTION_CAPTURE_GROUP = 256,

        const OPTION_NOTBOL = 512,
        const OPTION_NOTEOL = 1024,
        const OPTION_POSIX_REGION = 2048,
        const OPTION_MAXBIT = 4096
    }
}

pub struct Error {
    error: libc::c_int,
    description: String,
}

impl Error {
    fn new(error: libc::c_int, info: Option<ll::OnigErrorInfo>) -> Error {
        let mut err_buff = &mut [0 as u8; 90];
        let len  = unsafe {
            match info {
                Some(ref error_info) =>
                    ll::onig_error_code_to_str(
                        err_buff.as_mut_ptr(),
                        error,
                        error_info as *const ll::OnigErrorInfo
                    ),
                None => ll::onig_error_code_to_str(err_buff.as_mut_ptr(), error)
            }
        };
        Error {
            error: error,
            description: str::from_utf8(&err_buff[..len as usize]).unwrap().to_owned()
        }
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

#[derive(Debug)]
pub struct Region {
    raw: *const ll::OnigRegion
}

impl Region {
    pub fn new() -> Region {
        let raw = unsafe {
            ll::onig_region_new()
        };
        Region { raw:raw }
    }

    pub fn len(&self) -> usize {
        unsafe {
            (*self.raw).num_regs as usize
        }
    }

    pub unsafe fn unchecked_pos(&self, pos: usize) -> (usize, usize) {
        let beg = (*self.raw).beg.offset(pos as isize);
        let end = (*self.raw).end.offset(pos as isize);
        (*beg as usize, *end as usize)
    }

    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        if pos >= self.len() {
            return None
        }
        Some(unsafe {
            self.unchecked_pos(pos)
        })
    }

    pub fn clear(&mut self) {
        unsafe {
            ll::onig_region_clear(self.raw);
        }
    }

    pub fn resize(&mut self, new_size: usize) -> usize {
        unsafe {
            ll::onig_region_resize(self.raw, new_size as libc::c_int) as usize
        }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe {
            ll::onig_region_free(self.raw, 1);
        }
    }
}

#[derive(Debug)]
pub struct Regex {
    raw: ll::OnigRegex
}

impl Regex {
    pub fn new(
        pattern: &str,
        option: OptionType,
        syntax: *const ll::OnigSyntaxTypeStruct
            ) -> Result<Regex, Error> {

        // Convert the rust types to those required for the call to
        // `onig_new`.
        let pattern_bytes = pattern.as_bytes();
        let (start, end) = (
            pattern_bytes.as_ptr(),
            pattern_bytes[pattern_bytes.len()..].as_ptr()
        );
        let mut reg: ll::OnigRegex = ptr::null();
        let reg_ptr = &mut reg as *mut ll::OnigRegex;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = ll::OnigErrorInfo {
            enc: ptr::null(),
            par: ptr::null(),
            par_end: ptr::null()
        };

        let err = unsafe {
            ll::onig_new(
                reg_ptr,
                start,
                end,
                option.bits(),
                ENCODING_UTF8,
                syntax,
                &mut error)
        };

        if err == 0 {
            Ok(Regex{ raw: reg })
        } else {
            Err(Error::new(err, Some(error)))
        }
    }

    pub fn search(&self, text: &str, region: &mut Region, option: OptionType)
        -> Result<Option<usize>, Error> {
        let text_bytes = text.as_bytes();
        let (start, end) = (
            text_bytes.as_ptr(),
            text_bytes[text_bytes.len()..].as_ptr()
        );

        let r = unsafe {
            ll::onig_search(
                self.raw,
                start,
                end,
                start,
                end,
                region.raw,
                option.bits()
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
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe {
            ll::onig_free(self.raw);
        }
    }
}
