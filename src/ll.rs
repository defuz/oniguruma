use libc;

pub type OnigRegex = *const u8;
pub type OnigSyntaxTypeStruct = libc::c_void;
pub type OnigEncodingType = libc::c_void;
pub type OnigOptionTypeBits = libc::c_int;

#[repr(C)]
pub struct OnigErrorInfo {
    pub enc: *const OnigEncodingType,
    pub par: *const u8,
    pub par_end: *const u8
}

#[repr(C)]
pub struct OnigRegion {
    allocated: libc::c_int,
    pub num_regs: libc::c_int,
    pub beg: *const libc::c_int,
    pub end: *const libc::c_int,
    history_root: *const u8
}

#[link(name="onig")]
extern {
    pub static OnigSyntaxASIS: OnigSyntaxTypeStruct;
    pub static OnigSyntaxPosixBasic: OnigSyntaxTypeStruct;
    pub static OnigSyntaxPosixExtended: OnigSyntaxTypeStruct;
    pub static OnigSyntaxEmacs: OnigSyntaxTypeStruct;
    pub static OnigSyntaxGrep: OnigSyntaxTypeStruct;
    pub static OnigSyntaxGnuRegex: OnigSyntaxTypeStruct;
    pub static OnigSyntaxJava: OnigSyntaxTypeStruct;
    pub static OnigSyntaxPerl: OnigSyntaxTypeStruct;
    pub static OnigSyntaxPerl_NG: OnigSyntaxTypeStruct;
    pub static OnigSyntaxRuby: OnigSyntaxTypeStruct;

    pub static OnigEncodingUTF8: OnigEncodingType;

    pub fn onig_error_code_to_str(err_buff: *mut u8, err_code: libc::c_int, ...) -> libc::c_int;

    pub fn onig_new(
        reg: *mut OnigRegex,
        pattern: *const u8,
        pattern_end: *const u8,
        option: OnigOptionTypeBits,
        enc: *const OnigEncodingType,
        syntax: *const OnigSyntaxTypeStruct,
        err_info: *mut OnigErrorInfo
    ) -> libc::c_int;

    pub fn onig_search(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        start: *const u8,
        range: *const u8,
        region: *const OnigRegion,
        option: OnigOptionTypeBits
    ) -> libc::c_int;

    pub fn onig_match(
        reg: OnigRegex,
        str: *const u8,
        end: *const u8,
        at: *const u8,
        region: *const OnigRegion,
        option: OnigOptionTypeBits
    ) -> libc::c_int;

    pub fn onig_free(reg: OnigRegex);

    pub fn onig_region_new() -> *const OnigRegion;
    pub fn onig_region_free(region: *const OnigRegion, free_self: libc::c_int);
    pub fn onig_region_clear(region: *const OnigRegion);
}
