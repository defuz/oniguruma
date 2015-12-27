use libc::{c_uint, c_ulong};
use super::{SyntaxOperator, SyntaxBehavior, Options};

#[link(name="onig")]
extern {
    static OnigSyntaxASIS: Syntax;
    static OnigSyntaxPosixBasic: Syntax;
    static OnigSyntaxPosixExtended: Syntax;
    static OnigSyntaxEmacs: Syntax;
    static OnigSyntaxGrep: Syntax;
    static OnigSyntaxGnuRegex: Syntax;
    static OnigSyntaxJava: Syntax;
    static OnigSyntaxPerl: Syntax;
    static OnigSyntaxPerl_NG: Syntax;
    static OnigSyntaxRuby: Syntax;

    fn onig_copy_syntax(to: *const Syntax, from: *const Syntax);

    fn onig_get_syntax_op(syntax: *const Syntax) -> c_uint;
    fn onig_get_syntax_op2(syntax: *const Syntax) -> c_uint;
    fn onig_get_syntax_behavior(syntax: *const Syntax) -> c_uint;
    fn onig_get_syntax_options(syntax: *const Syntax) -> c_uint;

    fn onig_set_syntax_op(syntax: *mut Syntax, op: c_uint);
    fn onig_set_syntax_op2(syntax: *mut Syntax, op2: c_uint);
    fn onig_set_syntax_behavior(syntax: *mut Syntax, behavior: c_uint);
    fn onig_set_syntax_options(syntax: *mut Syntax, options: c_uint);
}

/// Plain text syntax
pub static SYNTAX_ASIS: &'static Syntax = &OnigSyntaxASIS;
/// POSIX Basic RE syntax
pub static SYNTAX_POSIX_BASIC: &'static Syntax = &OnigSyntaxPosixBasic;
/// POSIX Extended RE syntax
pub static SYNTAX_POSIX_EXTENDED: &'static Syntax = &OnigSyntaxPosixExtended;
/// Emacs syntax
pub static SYNTAX_EMACS: &'static Syntax = &OnigSyntaxEmacs;
/// Grep syntax
pub static SYNTAX_GREP: &'static Syntax = &OnigSyntaxGrep;
/// GNU regex syntax
pub static SYNTAX_GNU_REGEX: &'static Syntax = &OnigSyntaxGnuRegex;
/// Java (Sun java.util.regex) syntax
pub static SYNTAX_JAVA: &'static Syntax = &OnigSyntaxJava;
/// Perl syntax
pub static SYNTAX_PERL: &'static Syntax = &OnigSyntaxPerl;
/// Perl + named group syntax
pub static SYNTAX_PERL_NG: &'static Syntax = &OnigSyntaxPerl_NG;
/// Ruby syntax (default)
pub static SYNTAX_RUBY: &'static Syntax = &OnigSyntaxRuby;


type OnigCodePoint = c_ulong;

#[repr(C)]
#[derive(Debug)]
struct OnigMetaCharTable {
    esc: OnigCodePoint,
    anychar: OnigCodePoint,
    anytime: OnigCodePoint,
    zero_or_one_time: OnigCodePoint,
    one_or_more_time: OnigCodePoint,
    anychar_anytime: OnigCodePoint
}

#[repr(C)]
#[derive(Debug)]
pub struct Syntax {
    op: c_uint,
    op2: c_uint,
    behavior: c_uint,
    options: c_uint,
    meta_char_table: OnigMetaCharTable
}

impl Clone for Syntax {
    fn clone(&self) -> Syntax {
        let mut syntax = Syntax {
            op: 0,
            op2: 0,
            behavior: 0,
            options: 0,
            meta_char_table: OnigMetaCharTable {
                esc: 0,
                anychar: 0,
                anytime: 0,
                zero_or_one_time: 0,
                one_or_more_time: 0,
                anychar_anytime: 0,
            }
        };
        Syntax::clone_from(&mut syntax, self);
        syntax
    }

    fn clone_from(&mut self, source: &Syntax) {
        unsafe {
            onig_copy_syntax(self, source)
        }
    }
}

impl Syntax {
    pub fn get_operators(&self) -> SyntaxOperator {
        SyntaxOperator::from_bits_truncate(unsafe {
            onig_get_syntax_op(self) as u64 + ((onig_get_syntax_op2(self) as u64) << 32)
        })
    }

    pub fn set_operators(&mut self, operators: SyntaxOperator) {
        unsafe {
            onig_set_syntax_op(self, operators.bits() as c_uint);
            onig_set_syntax_op2(self, (operators.bits() >> 32) as c_uint)
        }
    }

    pub fn enable_operators(&mut self, operators: SyntaxOperator) {
        let operators = self.get_operators() | operators;
        self.set_operators(operators)
    }

    pub fn disable_operators(&mut self, operators: SyntaxOperator) {
        let operators = self.get_operators() & !operators;
        self.set_operators(operators)
    }

    pub fn get_behaviors(&self) -> SyntaxBehavior {
        SyntaxBehavior::from_bits_truncate(unsafe {
            onig_get_syntax_behavior(self)
        })
    }

    pub fn set_behaviors(&mut self, behaviors: SyntaxBehavior) {
        unsafe {
            onig_set_syntax_behavior(self, behaviors.bits() as c_uint);
        }
    }

    pub fn enable_behaviors(&mut self, behaviors: SyntaxBehavior) {
        let behaviors = self.get_behaviors() | behaviors;
        self.set_behaviors(behaviors)
    }

    pub fn disable_behaviors(&mut self, behaviors: SyntaxBehavior) {
        let behaviors = self.get_behaviors() & !behaviors;
        self.set_behaviors(behaviors)
    }

    pub fn get_options(&self) -> Options {
        Options::from_bits_truncate(unsafe {
            onig_get_syntax_options(self)
        })
    }

    pub fn set_options(&mut self, options: Options) {
        unsafe {
            onig_set_syntax_options(self, options.bits() as c_uint);
        }
    }
}
