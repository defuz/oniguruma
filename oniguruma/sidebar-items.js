initSidebarItems({"constant":[["OPTION_CAPTURE_GROUP","Named and no-named group captured. This is search time option."],["OPTION_DONT_CAPTURE_GROUP","Only named group captured. This is search time option."],["OPTION_EXTEND","Extended pattern form. This is compile time option."],["OPTION_FIND_LONGEST","Find longest match. This is compile time option."],["OPTION_FIND_NOT_EMPTY","Ignore empty match. This is compile time option."],["OPTION_IGNORECASE","Ambiguity match on. This is compile time option."],["OPTION_MULTILINE","`'.'` match with newline. This is compile time option."],["OPTION_NEGATE_SINGLELINE","Clear `OPTION_SINGLELINE` which is enabled on `SYNTAX_POSIX_BASIC`, `SYNTAX_POSIX_EXTENDED`, `SYNTAX_PERL`, `SYNTAX_PERL_NG`, `SYNTAX_JAVA`. This is compile time option."],["OPTION_NONE","Default options. This is both compile and search time option."],["OPTION_NOTBOL","String head isn't considered as begin of line"],["OPTION_NOTEOL","String end isn't considered as end of line"],["OPTION_SINGLELINE","`'^'` -> `'\\A'`, `'$'` -> `'\\Z'`. This is compile time option."],["SYNTAX_BEHAVIOR_ALLOW_DOUBLE_RANGE_OP_IN_CC","`[0-9-a]=[0-9-a]`"],["SYNTAX_BEHAVIOR_ALLOW_INTERVAL_LOW_ABBREV","`{,n} => {0,n}`"],["SYNTAX_BEHAVIOR_ALLOW_INVALID_INTERVAL","`{???`"],["SYNTAX_BEHAVIOR_ALLOW_MULTIPLEX_DEFINITION_NAME","`(?<x>)(?<x>)`"],["SYNTAX_BEHAVIOR_ALLOW_UNMATCHED_CLOSE_SUBEXP","`...)...`"],["SYNTAX_BEHAVIOR_BACKSLASH_ESCAPE_IN_CC","`[..\\w..] etc..`"],["SYNTAX_BEHAVIOR_CAPTURE_ONLY_NAMED_GROUP","See Oniguruma documenation"],["SYNTAX_BEHAVIOR_CONTEXT_INDEP_REPEAT_OPS","`?, *, +, {n,m}`"],["SYNTAX_BEHAVIOR_CONTEXT_INVALID_REPEAT_OPS","`error or ignore`"],["SYNTAX_BEHAVIOR_DIFFERENT_LEN_ALT_LOOK_BEHIND","`(?<=a|bc)`"],["SYNTAX_BEHAVIOR_FIXED_INTERVAL_IS_GREEDY_ONLY","`a{n}?=(?:a{n})?`"],["SYNTAX_BEHAVIOR_NOT_NEWLINE_IN_NEGATIVE_CC","`[^...]`"],["SYNTAX_BEHAVIOR_STRICT_CHECK_BACKREF","`/(\\1)/,/\\1()/ ..`"],["SYNTAX_BEHAVIOR_WARN_CC_OP_NOT_ESCAPED","`[,-,]`"],["SYNTAX_BEHAVIOR_WARN_REDUNDANT_NESTED_REPEAT","`(?:a*)+`"],["SYNTAX_OPERATOR_ASTERISK_ZERO_INF","`*`"],["SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY","`(?@..),(?@<x>..)`"],["SYNTAX_OPERATOR_BRACE_INTERVAL","`{lower,upper}`"],["SYNTAX_OPERATOR_BRACKET_CC","`[...]`"],["SYNTAX_OPERATOR_CCLASS_SET_OP","`[...&&..[..]..]`"],["SYNTAX_OPERATOR_DECIMAL_BACKREF","`\\num`"],["SYNTAX_OPERATOR_DOT_ANYCHAR","`.`"],["SYNTAX_OPERATOR_ESC_AZ_BUF_ANCHOR","`\\A, \\Z, \\z`"],["SYNTAX_OPERATOR_ESC_BRACE_INTERVAL","`{lower,upper}`"],["SYNTAX_OPERATOR_ESC_B_WORD_BOUND","`\\b, \\B`"],["SYNTAX_OPERATOR_ESC_CAPITAL_C_BAR_CONTROL","`\\C-x`"],["SYNTAX_OPERATOR_ESC_CAPITAL_G_BEGIN_ANCHOR","`\\G`"],["SYNTAX_OPERATOR_ESC_CAPITAL_M_BAR_META","`\\M-x`"],["SYNTAX_OPERATOR_ESC_CAPITAL_Q_QUOTE","`\\Q...\\E`"],["SYNTAX_OPERATOR_ESC_CONTROL_CHARS","`\\n,\\r,\\t,\\a ...`"],["SYNTAX_OPERATOR_ESC_C_CONTROL","`\\cx`"],["SYNTAX_OPERATOR_ESC_D_DIGIT","`\\d, \\D`"],["SYNTAX_OPERATOR_ESC_GNU_BUF_ANCHOR","``, \\'`"],["SYNTAX_OPERATOR_ESC_G_SUBEXP_CALL","`\\g<name>, \\g<n>`"],["SYNTAX_OPERATOR_ESC_H_XDIGIT","`\\h, \\H`"],["SYNTAX_OPERATOR_ESC_K_NAMED_BACKREF","`\\k<name>`"],["SYNTAX_OPERATOR_ESC_LPAREN_SUBEXP","`(...)`"],["SYNTAX_OPERATOR_ESC_LTGT_WORD_BEGIN_END","`<. >`"],["SYNTAX_OPERATOR_ESC_OCTAL3","`\\OOO`"],["SYNTAX_OPERATOR_ESC_P_BRACE_CHAR_PROPERTY","`\\p{...}, \\P{...}`"],["SYNTAX_OPERATOR_ESC_P_BRACE_CIRCUMFLEX_NOT","`\\p{^..}, \\P{^..}`"],["SYNTAX_OPERATOR_ESC_S_WHITE_SPACE","`\\s, \\S`"],["SYNTAX_OPERATOR_ESC_U_HEX4","`\\uHHHH`"],["SYNTAX_OPERATOR_ESC_VBAR_ALT","`|`"],["SYNTAX_OPERATOR_ESC_V_VTAB","`\\v as VTAB`"],["SYNTAX_OPERATOR_ESC_W_WORD","`\\w, \\W`"],["SYNTAX_OPERATOR_ESC_X_BRACE_HEX8","`\\x{7HHHHHHH}`"],["SYNTAX_OPERATOR_ESC_X_HEX2","`\\xHH`"],["SYNTAX_OPERATOR_INEFFECTIVE_ESCAPE","``"],["SYNTAX_OPERATOR_LINE_ANCHOR","`^, $`"],["SYNTAX_OPERATOR_LPAREN_SUBEXP","`(...)`"],["SYNTAX_OPERATOR_OPTION_PERL","`(?imsx),(?-imsx)`"],["SYNTAX_OPERATOR_OPTION_RUBY","`(?imx), (?-imx)`"],["SYNTAX_OPERATOR_PLUS_ONE_INF","`+`"],["SYNTAX_OPERATOR_PLUS_POSSESSIVE_INTERVAL","`{n,m}+`"],["SYNTAX_OPERATOR_PLUS_POSSESSIVE_REPEAT","`?+,*+,++`"],["SYNTAX_OPERATOR_POSIX_BRACKET","`[:xxxx:]`"],["SYNTAX_OPERATOR_QMARK_GROUP_EFFECT","`(?...)`"],["SYNTAX_OPERATOR_QMARK_LT_NAMED_GROUP","`(?<name>...)`"],["SYNTAX_OPERATOR_QMARK_NON_GREEDY","`??,*?,+?,{n,m}?`"],["SYNTAX_OPERATOR_QMARK_ZERO_ONE","`?`"],["SYNTAX_OPERATOR_VBAR_ALT","`|`"]],"static":[["ENCODING_UTF8",""],["SYNTAX_ASIS","Plain text syntax"],["SYNTAX_EMACS","Emacs syntax"],["SYNTAX_GNU_REGEX","GNU regex syntax"],["SYNTAX_GREP","Grep syntax"],["SYNTAX_JAVA","Java (Sun java.util.regex) syntax"],["SYNTAX_PERL","Perl syntax"],["SYNTAX_PERL_NG","Perl + named group syntax"],["SYNTAX_POSIX_BASIC","POSIX Basic RE syntax"],["SYNTAX_POSIX_EXTENDED","POSIX Extended RE syntax"],["SYNTAX_RUBY","Ruby syntax (default)"]],"struct":[["CaptureTreeNode",""],["CaptureTreeNodeIter",""],["Captures","Captures represents a group of captured strings for a single match."],["Error","An error that occurred during parsing, compiling or evaluating a regular expression."],["Options","Regex parsing, compilation and evaluation options."],["Regex","A compiled Oniguruma regular expression."],["RegexConfig",""],["Region","Representation of regex search result."],["SubCaptures","An iterator over capture groups for a particular match of a regular  expression."],["SubCapturesPos","An iterator over capture group positions for a particular match of a regular expression."],["Syntax",""],["SyntaxBehavior",""],["SyntaxOperator",""]],"type":[["Encoding",""]]});