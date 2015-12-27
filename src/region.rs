use libc::c_int;
use std::{iter, ops, mem, ptr};

#[link(name="onig")]
extern {
    // TODO: shoud we use original malloc memory managment for regions?
    // fn onig_region_new() -> *const Region;
    // fn onig_region_free(region: *const Region, free_self: c_int);
    fn onig_region_clear(region: *const Region);
    fn onig_get_capture_tree(region: *const Region) -> *const CaptureTreeNode;
}

#[repr(C)]
#[derive(Debug)]
pub struct CaptureTreeNode {
    group: c_int,
    beg: c_int,
    end: c_int,
    allocated: c_int,
    num_childs: c_int,
    childs: *const *const CaptureTreeNode
}

/// Representation of regex search result.
#[repr(C)]
#[derive(Debug)]
pub struct Region {
    allocated: c_int,
    num_regs: c_int,
    beg: *const c_int,
    end: *const c_int,
    history_root: *const CaptureTreeNode
}

impl Region {
    /// Create empty region.
    pub fn new() -> Region {
        Region {
            allocated: 0,
            num_regs: 0,
            beg: ptr::null(),
            end: ptr::null(),
            history_root: ptr::null()
        }
    }

    /// Returns the number of captured groups.
    pub fn len(&self) -> usize {
        self.num_regs as usize
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
                *self.beg.offset(pos as isize),
                *self.end.offset(pos as isize)
            )
        };
        if beg >= 0 {
            Some((beg as usize, end as usize))
        } else {
            None
        }
    }

    pub fn tree(&self) -> Option<&CaptureTreeNode> {
        let raw = unsafe {
            onig_get_capture_tree(self)
        };
        if raw.is_null() {
            None
        } else {
            Some(unsafe {
                mem::transmute(raw)
            })
        }
    }

    /// Clear contents of region.
    pub fn clear(&mut self) {
        unsafe {
            onig_region_clear(self);
        }
    }
}

impl CaptureTreeNode {
    pub fn group(&self) -> usize {
        self.group as usize
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.beg as usize, self.end as usize)
    }

    pub fn len(&self) -> usize {
        self.num_childs as usize
    }

    pub fn childs<'t>(&'t self) -> CaptureTreeNodeIter<'t> {
        CaptureTreeNodeIter { idx: 0, node: self }
    }
}

impl ops::Index<usize> for CaptureTreeNode {
    type Output = CaptureTreeNode;

    fn index(&self, index: usize) -> &CaptureTreeNode {
        if index >= self.len() {
            panic!("capture tree node index overflow")
        }
        unsafe {
            mem::transmute(*self.childs.offset(index as isize))
        }
    }
}

#[derive(Debug)]
pub struct CaptureTreeNodeIter<'t> {
    idx: usize,
    node: &'t CaptureTreeNode
}

impl<'t> iter::Iterator for CaptureTreeNodeIter<'t> {
    type Item = &'t CaptureTreeNode;

    fn next(&mut self) -> Option<&'t CaptureTreeNode> {
        if self.idx < self.node.len() {
            self.idx += 1;
            Some(&self.node[self.idx - 1])
        } else {
            None
        }
    }
}
