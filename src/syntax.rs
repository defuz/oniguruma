use libc::{c_uint, c_ulong};

use super::Options;

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
    pub esc: OnigCodePoint,
    pub anychar: OnigCodePoint,
    pub anytime: OnigCodePoint,
    pub zero_or_one_time: OnigCodePoint,
    pub one_or_more_time: OnigCodePoint,
    pub anychar_anytime: OnigCodePoint
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

bitflags! {
    flags SyntaxOperator: u64 {
        /// `.`
        const SYNTAX_OPERATOR_DOT_ANYCHAR                 = 1u64 << 1,
        /// `*`
        const SYNTAX_OPERATOR_ASTERISK_ZERO_INF           = 1u64 << 2,
        /// `+`
        const SYNTAX_OPERATOR_PLUS_ONE_INF                = 1u64 << 4,
        /// `?`
        const SYNTAX_OPERATOR_QMARK_ZERO_ONE              = 1u64 << 6,
        /// `{lower,upper}`
        const SYNTAX_OPERATOR_BRACE_INTERVAL              = 1u64 << 8,
        /// `\{lower,upper\}`
        const SYNTAX_OPERATOR_ESC_BRACE_INTERVAL          = 1u64 << 9,
        /// `|`
        const SYNTAX_OPERATOR_VBAR_ALT                    = 1u64 << 10,
        /// `\|`
        const SYNTAX_OPERATOR_ESC_VBAR_ALT                = 1u64 << 11,
        /// `(...)`
        const SYNTAX_OPERATOR_LPAREN_SUBEXP               = 1u64 << 12,
        /// `\(...\)`
        const SYNTAX_OPERATOR_ESC_LPAREN_SUBEXP           = 1u64 << 13,
        /// `\A, \Z, \z`
        const SYNTAX_OPERATOR_ESC_AZ_BUF_ANCHOR           = 1u64 << 14,
        /// `\G`
        const SYNTAX_OPERATOR_ESC_CAPITAL_G_BEGIN_ANCHOR  = 1u64 << 15,
        /// `\num`
        const SYNTAX_OPERATOR_DECIMAL_BACKREF             = 1u64 << 16,
        /// `[...]`
        const SYNTAX_OPERATOR_BRACKET_CC                  = 1u64 << 17,
        /// `\w, \W`
        const SYNTAX_OPERATOR_ESC_W_WORD                  = 1u64 << 18,
        /// `\<. \>`
        const SYNTAX_OPERATOR_ESC_LTGT_WORD_BEGIN_END     = 1u64 << 19,
        /// `\b, \B`
        const SYNTAX_OPERATOR_ESC_B_WORD_BOUND            = 1u64 << 20,
        /// `\s, \S`
        const SYNTAX_OPERATOR_ESC_S_WHITE_SPACE           = 1u64 << 21,
        /// `\d, \D`
        const SYNTAX_OPERATOR_ESC_D_DIGIT                 = 1u64 << 22,
        /// `^, $`
        const SYNTAX_OPERATOR_LINE_ANCHOR                 = 1u64 << 23,
        /// `[:xxxx:]`
        const SYNTAX_OPERATOR_POSIX_BRACKET               = 1u64 << 24,
        /// `??,*?,+?,{n,m}?`
        const SYNTAX_OPERATOR_QMARK_NON_GREEDY            = 1u64 << 25,
        /// `\n,\r,\t,\a ...`
        const SYNTAX_OPERATOR_ESC_CONTROL_CHARS           = 1u64 << 26,
        /// `\cx`
        const SYNTAX_OPERATOR_ESC_C_CONTROL               = 1u64 << 27,
        /// `\OOO`
        const SYNTAX_OPERATOR_ESC_OCTAL3                  = 1u64 << 28,
        /// `\xHH`
        const SYNTAX_OPERATOR_ESC_X_HEX2                  = 1u64 << 29,
        /// `\x{7HHHHHHH}`
        const SYNTAX_OPERATOR_ESC_X_BRACE_HEX8            = 1u64 << 30,
        /// `\Q...\E`
        const SYNTAX_OPERATOR_ESC_CAPITAL_Q_QUOTE         = 1u64 << (32 + 0),
        /// `(?...)`
        const SYNTAX_OPERATOR_QMARK_GROUP_EFFECT          = 1u64 << (32 + 1),
        /// `(?imsx),(?-imsx)`
        const SYNTAX_OPERATOR_OPTION_PERL                 = 1u64 << (32 + 2),
        /// `(?imx), (?-imx)`
        const SYNTAX_OPERATOR_OPTION_RUBY                 = 1u64 << (32 + 3),
        /// `?+,*+,++`
        const SYNTAX_OPERATOR_PLUS_POSSESSIVE_REPEAT      = 1u64 << (32 + 4),
        /// `{n,m}+`
        const SYNTAX_OPERATOR_PLUS_POSSESSIVE_INTERVAL    = 1u64 << (32 + 5),
        /// `[...&&..[..]..]`
        const SYNTAX_OPERATOR_CCLASS_SET_OP               = 1u64 << (32 + 6),
        /// `(?<name>...)`
        const SYNTAX_OPERATOR_QMARK_LT_NAMED_GROUP        = 1u64 << (32 + 7),
        /// `\k<name>`
        const SYNTAX_OPERATOR_ESC_K_NAMED_BACKREF         = 1u64 << (32 + 8),
        /// `\g<name>, \g<n>`
        const SYNTAX_OPERATOR_ESC_G_SUBEXP_CALL           = 1u64 << (32 + 9),
        /// `(?@..),(?@<x>..)`
        const SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY      = 1u64 << (32 + 10),
        /// `\C-x`
        const SYNTAX_OPERATOR_ESC_CAPITAL_C_BAR_CONTROL   = 1u64 << (32 + 11),
        /// `\M-x`
        const SYNTAX_OPERATOR_ESC_CAPITAL_M_BAR_META      = 1u64 << (32 + 12),
        /// `\v as VTAB`
        const SYNTAX_OPERATOR_ESC_V_VTAB                  = 1u64 << (32 + 13),
        /// `\uHHHH`
        const SYNTAX_OPERATOR_ESC_U_HEX4                  = 1u64 << (32 + 14),
        /// `\`, \'`
        const SYNTAX_OPERATOR_ESC_GNU_BUF_ANCHOR          = 1u64 << (32 + 15),
        /// `\p{...}, \P{...}`
        const SYNTAX_OPERATOR_ESC_P_BRACE_CHAR_PROPERTY   = 1u64 << (32 + 16),
        /// `\p{^..}, \P{^..}`
        const SYNTAX_OPERATOR_ESC_P_BRACE_CIRCUMFLEX_NOT  = 1u64 << (32 + 17),
        /// `\h, \H`
        const SYNTAX_OPERATOR_ESC_H_XDIGIT                = 1u64 << (32 + 19),
        /// `\`
        const SYNTAX_OPERATOR_INEFFECTIVE_ESCAPE          = 1u64 << (32 + 20)
    }
}

bitflags! {
    flags SyntaxBehavior: u32 {
        /// `?, *, +, {n,m}`
        const SYNTAX_BEHAVIOR_CONTEXT_INDEP_REPEAT_OPS        = 1u32 << 0,
        /// `error or ignore`
        const SYNTAX_BEHAVIOR_CONTEXT_INVALID_REPEAT_OPS      = 1u32 << 1,
        /// `...)...`
        const SYNTAX_BEHAVIOR_ALLOW_UNMATCHED_CLOSE_SUBEXP    = 1u32 << 2,
        /// `{???`
        const SYNTAX_BEHAVIOR_ALLOW_INVALID_INTERVAL          = 1u32 << 3,
        /// `{,n} => {0,n}`
        const SYNTAX_BEHAVIOR_ALLOW_INTERVAL_LOW_ABBREV       = 1u32 << 4,
        /// `/(\1)/,/\1()/ ..`
        const SYNTAX_BEHAVIOR_STRICT_CHECK_BACKREF            = 1u32 << 5,
        /// `(?<=a|bc)`
        const SYNTAX_BEHAVIOR_DIFFERENT_LEN_ALT_LOOK_BEHIND   = 1u32 << 6,
        /// See Oniguruma documenation
        const SYNTAX_BEHAVIOR_CAPTURE_ONLY_NAMED_GROUP        = 1u32 << 7,
        /// `(?<x>)(?<x>)`
        const SYNTAX_BEHAVIOR_ALLOW_MULTIPLEX_DEFINITION_NAME = 1u32 << 8,
        /// `a{n}?=(?:a{n})?`
        const SYNTAX_BEHAVIOR_FIXED_INTERVAL_IS_GREEDY_ONLY   = 1u32 << 9,
        /// `[^...]`
        const SYNTAX_BEHAVIOR_NOT_NEWLINE_IN_NEGATIVE_CC      = 1u32 << 20,
        /// `[..\w..] etc..`
        const SYNTAX_BEHAVIOR_BACKSLASH_ESCAPE_IN_CC          = 1u32 << 21,
        /// `[0-9-a]=[0-9\-a]`
        const SYNTAX_BEHAVIOR_ALLOW_DOUBLE_RANGE_OP_IN_CC     = 1u32 << 23,
        /// `[,-,]`
        const SYNTAX_BEHAVIOR_WARN_CC_OP_NOT_ESCAPED          = 1u32 << 24,
        /// `(?:a*)+`
        const SYNTAX_BEHAVIOR_WARN_REDUNDANT_NESTED_REPEAT    = 1u32 << 25
    }
}
