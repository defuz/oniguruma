use libc::c_void;

#[link(name="onig")]
extern {
    static OnigEncodingUTF8: Encoding;
}

pub type Encoding = c_void;
pub static ENCODING_UTF8: &'static Encoding = &OnigEncodingUTF8;
