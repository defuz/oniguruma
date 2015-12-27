//! Rust bindings for the [Oniguruma](https://github.com/kkos/oniguruma)
//! regular expressions library.
//!
//! Example of usage:
//!
//! ```rust
//! use oniguruma::Regex;
//!
//! let regex = Regex::new("e(l+)").unwrap();
//! for (i, pos) in regex.captures("hello").unwrap().iter_pos().enumerate() {
//!     match pos {
//!          Some((beg, end)) =>
//!              println!("Group {} captured in position {}:{}", i, beg, end),
//!          None =>
//!              println!("Group {} is not captured", i)
//!     }
//! }
//! ```
extern crate libc;

#[macro_use]
extern crate bitflags;

mod ll;

#[cfg(test)]
mod test;

use std::{error, fmt, str, ptr, iter, marker};

/// Plain text syntax
pub static SYNTAX_ASIS: *const ll::OnigSyntax
    = &ll::OnigSyntaxASIS;
/// POSIX Basic RE syntax
pub static SYNTAX_POSIX_BASIC: *const ll::OnigSyntax
    = &ll::OnigSyntaxPosixBasic;
/// POSIX Extended RE syntax
pub static SYNTAX_POSIX_EXTENDED: *const ll::OnigSyntax
    = &ll::OnigSyntaxPosixExtended;
/// Emacs syntax
pub static SYNTAX_EMACS: *const ll::OnigSyntax
    = &ll::OnigSyntaxEmacs;
/// Grep syntax
pub static SYNTAX_GREP: *const ll::OnigSyntax
    = &ll::OnigSyntaxGrep;
/// GNU regex syntax
pub static SYNTAX_GNU_REGEX: *const ll::OnigSyntax
    = &ll::OnigSyntaxGnuRegex;
/// Java (Sun java.util.regex) syntax
pub static SYNTAX_JAVA: *const ll::OnigSyntax
    = &ll::OnigSyntaxJava;
/// Perl syntax
pub static SYNTAX_PERL: *const ll::OnigSyntax
    = &ll::OnigSyntaxPerl;
/// Perl + named group syntax
pub static SYNTAX_PERL_NG: *const ll::OnigSyntax
    = &ll::OnigSyntaxPerl_NG;
/// Ruby (default) syntax
pub static SYNTAX_RUBY: *const ll::OnigSyntax
    = &ll::OnigSyntaxRuby;

pub static ENCODING_UTF8: *const ll::OnigEncoding
    = &ll::OnigEncodingUTF8;

bitflags!{
    /// Regex parsing, compilation and evaluation options.
    flags Options: ll::OnigOptions {
        /// Default options. This is both compile and search time option.
        const OPTION_NONE = 0,
        /// Ambiguity match on. This is compile time option.
        const OPTION_IGNORECASE = 1,
        /// Extended pattern form. This is compile time option.
        const OPTION_EXTEND = 2,
        /// `'.'` match with newline. This is compile time option.
        const OPTION_MULTILINE = 4,
        /// `'^'` -> `'\A'`, `'$'` -> `'\Z'`. This is compile time option.
        const OPTION_SINGLELINE = 8,
        /// Find longest match. This is compile time option.
        const OPTION_FIND_LONGEST = 16,
        /// Ignore empty match. This is compile time option.
        const OPTION_FIND_NOT_EMPTY = 32,
        /// Clear `OPTION_SINGLELINE` which is enabled on
        /// `SYNTAX_POSIX_BASIC`, `SYNTAX_POSIX_EXTENDED`,
        /// `SYNTAX_PERL`, `SYNTAX_PERL_NG`, `SYNTAX_JAVA`.
        /// This is compile time option.
        const OPTION_NEGATE_SINGLELINE = 64,
        /// Only named group captured. This is search time option.
        const OPTION_DONT_CAPTURE_GROUP = 128,
        /// Named and no-named group captured. This is search time option.
        const OPTION_CAPTURE_GROUP = 256,

        /// String head isn't considered as begin of line
        const OPTION_NOTBOL = 512,
        /// String end isn't considered as end of line
        const OPTION_NOTEOL = 1024,
        // const OPTION_POSIX_REGION = 2048,
        // const OPTION_MAXBIT = 4096
    }
}

/// An error that occurred during parsing, compiling or evaluating
/// a regular expression.
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

/// Representation of regex search result.
#[derive(Debug)]
pub struct Region {
    raw: *const ll::OnigRegion
}

impl Region {
    /// Create empty region.
    pub fn new() -> Region {
        let raw = unsafe {
            ll::onig_region_new()
        };
        Region { raw:raw }
    }

    /// Returns the number of captured groups.
    pub fn len(&self) -> usize {
        unsafe {
            (*self.raw).num_regs as usize
        }
    }

    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if i is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are always byte indices with
    /// respect to the original string matched.
    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        if pos >= self.len() {
            return None
        }
        let (beg, end) = unsafe {
            (
                *(*self.raw).beg.offset(pos as isize),
                *(*self.raw).end.offset(pos as isize)
            )
        };
        if beg >= 0 {
            Some((beg as usize, end as usize))
        } else {
            None
        }
    }

    pub fn tree<'r>(&'r self) -> Option<CaptureTreeNode<'r>> {
        let raw = unsafe {
            (*self.raw).history_root
            // ll::onig_get_capture_tree(self.raw)
        };
        if raw.is_null() {
            None
        } else {
            Some(CaptureTreeNode {
                raw: raw,
                region: marker::PhantomData
            })
        }
    }

    /// Clear contents of region.
    pub fn clear(&mut self) {
        unsafe {
            ll::onig_region_clear(self.raw);
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

#[derive(Debug, Clone, Copy)]
pub struct CaptureTreeNode<'r> {
    raw: *const ll::OnigCaptureTreeNode,
    region: marker::PhantomData<&'r Region>
}

impl<'r> CaptureTreeNode<'r> {
    pub fn group(&self) -> usize {
        unsafe {
            (*self.raw).group as usize
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        unsafe {
            ((*self.raw).beg as usize, (*self.raw).end as usize)
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            (*self.raw).num_childs as usize
        }
    }

    pub fn childs(self) -> CaptureTreeNodeIter<'r> {
        CaptureTreeNodeIter { idx: 0, node: self }
    }
}

#[derive(Debug)]
pub struct CaptureTreeNodeIter<'r> {
    idx: usize,
    node: CaptureTreeNode<'r>
}

impl<'r> iter::Iterator for CaptureTreeNodeIter<'r> {
    type Item = CaptureTreeNode<'r>;

    fn next(&mut self) -> Option<CaptureTreeNode<'r>> {
        if self.idx < self.node.len() {
            self.idx += 1;
            Some(CaptureTreeNode {
                raw: unsafe {
                    *(*self.node.raw).childs.offset((self.idx - 1) as isize)
                },
                region: self.node.region
            })
        } else {
            None
        }
    }
}

/// Captures represents a group of captured strings for a single match.
///
/// The 0th capture always corresponds to the entire match. Each subsequent
/// index corresponds to the next capture group in the regex. Positions
/// returned from a capture group are always byte indices.
///
/// `'t` is the lifetime of the matched text.
#[derive(Debug)]
pub struct Captures<'t> {
    text: &'t str,
    region: Region
}

impl<'t> Captures<'t> {
    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if i is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are always byte indices with
    /// respect to the original string matched.
    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        self.region.pos(pos)
    }

    /// Returns the matched string for the capture group `i`. If `i` isn't
    /// a valid capture group or didn't match anything, then `None` is returned.
    pub fn at(&self, pos: usize) -> Option<&'t str> {
        self.pos(pos).map(|(beg, end)| &self.text[beg..end])
    }

    /// Returns the number of captured groups.
    pub fn len(&self) -> usize {
        self.region.len()
    }

    /// Returns true if and only if there are no captured groups.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates an iterator of all the capture groups in order of appearance in
    // the regular expression.
    pub fn iter(&'t self) -> SubCaptures<'t> {
        SubCaptures { idx: 0, caps: self }
    }

    /// Creates an iterator of all the capture group positions in order of
    /// appearance in the regular expression. Positions are byte indices in
    /// terms of the original string matched.
    pub fn iter_pos(&'t self) -> SubCapturesPos<'t> {
        SubCapturesPos { idx: 0, caps: self }
    }
}

/// An iterator over capture groups for a particular match of a regular
/// expression.
///
///`'t` is the lifetime of the matched text.
pub struct SubCaptures<'t> {
    idx: usize,
    caps: &'t Captures<'t>
}

impl<'t> iter::Iterator for SubCaptures<'t> {
    type Item = Option<&'t str>;

    fn next(&mut self) -> Option<Option<&'t str>> {
        if self.idx < self.caps.len() {
            self.idx += 1;
            Some(self.caps.at(self.idx - 1))
        } else {
            None
        }
    }
}

/// An iterator over capture group positions for a particular match of
/// a regular expression.
///
/// Positions are byte indices in terms of the original
/// string matched. `'t` is the lifetime of the matched text.
pub struct SubCapturesPos<'t> {
    idx: usize,
    caps: &'t Captures<'t>
}

impl<'t> iter::Iterator for SubCapturesPos<'t> {
    type Item = Option<(usize, usize)>;

    fn next(&mut self) -> Option<Option<(usize, usize)>> {
        if self.idx < self.caps.len() {
            self.idx += 1;
            Some(self.caps.pos(self.idx - 1))
        } else {
            None
        }
    }
}

/// A compiled Oniguruma regular expression.
#[derive(Debug)]
pub struct Regex {
    raw: ll::OnigRegex
}

impl Regex {
    /// Compiles a regular expression with default options. Default syntax is
    /// `SYNTAX_RUBY`.
    ///
    /// Once compiled, it can be used repeatedly to search in a string. If an
    /// invalid expression is given, then an error is returned.
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        Regex::new_with_options_and_syntax(pattern, OPTION_NONE, SYNTAX_RUBY)
    }

    pub fn new_with_options(pattern: &str,
                           options: Options)
                           -> Result<Regex, Error> {
        Regex::new_with_options_and_syntax(pattern, options, SYNTAX_RUBY)
    }

    pub fn new_with_syntax(pattern: &str, syntax:
                           *const ll::OnigSyntax)
                           -> Result<Regex, Error> {
        Regex::new_with_options_and_syntax(pattern, OPTION_NONE, syntax)
    }

    pub fn new_with_options_and_syntax(pattern: &str,
                                      options: Options,
                                      syntax: *const ll::OnigSyntax)
                                      -> Result<Regex, Error> {

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
                options.bits(),
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
            ll::onig_search(
                self.raw,
                start,
                end,
                start,
                end,
                region.raw,
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
            ll::onig_match(
                self.raw,
                start,
                end,
                start,
                region.raw,
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

    /// Returns the capture groups corresponding to the leftmost-first match
    /// in text. Capture group `0` always corresponds to the entire match.
    /// If no match is found, then `None` is returned.
    ///
    /// # Panics
    ///
    /// This method may panic in the case of memory overflow during execution or
    /// other internal errors of Oniguruma engine.
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        let mut region = Region::new();
        self.search_with_region(text, &mut region, OPTION_NONE)
            .unwrap()
            .map(|_| Captures { text: text, region: region })
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
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe {
            ll::onig_free(self.raw);
        }
    }
}
