extern crate libc;

#[macro_use]
extern crate bitflags;

mod ll;

#[cfg(test)]
mod test;

use std::fmt;
use std::str;
use std::ptr::null;

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

pub static UTF8: *const ll::OnigEncodingType = &ll::OnigEncodingUTF8;

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
    error_info: Option<ll::OnigErrorInfo>
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err_buff = &mut [0 as u8; 1024];
        let len  = unsafe {
            match self.error_info {
                Some(ref error_info) =>
                    ll::onig_error_code_to_str(
                        err_buff.as_mut_ptr(),
                        self.error,
                        error_info as *const ll::OnigErrorInfo),
                None =>
                    ll::onig_error_code_to_str(
                        err_buff.as_mut_ptr(), self.error)
            }
        };
        let err_str_slice = str::from_utf8(&err_buff[..len as usize]).unwrap();
        write!(f, "Oniguruma error: {}", err_str_slice)
    }
}

#[allow(raw_pointer_derive)]
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

    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        if pos >= self.len() {
            return None
        }
        unsafe {
            let beg = (*self.raw).beg.offset(pos as isize);
            let end = (*self.raw).end.offset(pos as isize);
            Some((*beg as usize, *end as usize))
        }
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

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Regex {
    raw: *const ll::OnigRegex
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
        let mut reg: *const ll::OnigRegex = null();
        let reg_ptr = &mut reg as *mut *const ll::OnigRegex;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = ll::OnigErrorInfo {
            enc: null(),
            par: null(),
            par_end: null()
        };

        let err = unsafe {
            ll::onig_new(
                reg_ptr,
                start,
                end,
                option.bits(),
                UTF8,
                syntax,
                &mut error)
        };

        if err == 0 {
            Ok(Regex{ raw: reg })
        } else {
            Err(Error{ error: err, error_info: Some(error) })
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
            Err(Error { error: r, error_info: None })
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
