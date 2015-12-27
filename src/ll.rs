use libc;

pub type OnigRegex = *const u8;
pub type OnigSyntax = libc::c_void;
pub type OnigEncoding = libc::c_void;
pub type OnigOptions = libc::c_int;

#[repr(C)]
pub struct OnigErrorInfo {
    pub enc: *const OnigEncoding,
    pub par: *const u8,
    pub par_end: *const u8
}

#[repr(C)]
pub struct OnigRegion {
    allocated: libc::c_int,
    pub num_regs: libc::c_int,
    pub beg: *const libc::c_int,
    pub end: *const libc::c_int,
    pub history_root: *const OnigCaptureTreeNode
}

#[repr(C)]
pub struct OnigCaptureTreeNode {
    pub group: libc::c_int,
    pub beg: libc::c_int,
    pub end: libc::c_int,
    allocated: libc::c_int,
    pub num_childs: libc::c_int,
    pub childs: *const *const OnigCaptureTreeNode
}

#[link(name="onig")]
extern {
    pub static OnigSyntaxASIS: OnigSyntax;
    pub static OnigSyntaxPosixBasic: OnigSyntax;
    pub static OnigSyntaxPosixExtended: OnigSyntax;
    pub static OnigSyntaxEmacs: OnigSyntax;
    pub static OnigSyntaxGrep: OnigSyntax;
    pub static OnigSyntaxGnuRegex: OnigSyntax;
    pub static OnigSyntaxJava: OnigSyntax;
    pub static OnigSyntaxPerl: OnigSyntax;
    pub static OnigSyntaxPerl_NG: OnigSyntax;
    pub static OnigSyntaxRuby: OnigSyntax;

    pub static OnigEncodingUTF8: OnigEncoding;

    pub fn onig_error_code_to_str(err_buff: *mut u8, err_code: libc::c_int, ...) -> libc::c_int;

    pub fn onig_new(
        reg: *mut OnigRegex,
        pattern: *const u8,
        pattern_end: *const u8,
        option: OnigOptions,
        enc: *const OnigEncoding,
        syntax: *const OnigSyntax,
        err_info: *mut OnigErrorInfo
    ) -> libc::c_int;

    pub fn onig_search(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        start: *const u8,
        range: *const u8,
        region: *const OnigRegion,
        option: OnigOptions
    ) -> libc::c_int;

    pub fn onig_match(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        at: *const u8,
        region: *const OnigRegion,
        option: OnigOptions
    ) -> libc::c_int;

    pub fn onig_free(reg: OnigRegex);

    pub fn onig_region_new() -> *const OnigRegion;
    pub fn onig_region_free(region: *const OnigRegion, free_self: libc::c_int);
    pub fn onig_region_clear(region: *const OnigRegion);
    pub fn onig_get_capture_tree(region: *const OnigRegion) -> *const OnigCaptureTreeNode;
}
