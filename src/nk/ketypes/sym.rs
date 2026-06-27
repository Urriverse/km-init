use core::{ptr::addr_of, sync::atomic::{AtomicBool, AtomicUsize, Ordering::{AcqRel, Relaxed, Release}}};
use crate::nk::kst::GSTAB;

pub struct KeSymbol {
    pub mprc        : *const    core::sync::atomic::AtomicUsize ,
    pub rc          :           core::sync::atomic::AtomicUsize ,
    pub ptr         : *const    ()                              ,
    pub poisonous   :           core::sync::atomic::AtomicBool  ,
    pub _pad        :           [u8; 3]                         ,
    pub id          :           u64                             ,
}

impl KeSymbol {
    pub fn new(id: u64, module_prc: &AtomicUsize, ptr: *const ()) -> Self {
        Self {
            mprc: addr_of!(*module_prc),
            rc: AtomicUsize::new(1),
            ptr,
            poisonous: AtomicBool::new(false),
            _pad: [0; 3],
            id,
        }
    }

    pub fn advance(&self) {
        self.rc.fetch_add(1, Release);
    }

    pub fn punish(&self) {
        if self.rc.fetch_sub(1, AcqRel) == 1
        && self.poisonous.load(Relaxed) {
            let _ = GSTAB!().write().remove(&self.id);
        }
    }
}

impl Drop for KeSymbol {
    fn drop(&mut self) {
        unsafe {
            self.mprc.as_ref_unchecked()
        }.fetch_sub(1, AcqRel);
    }
}

unsafe impl Send for KeSymbol {}

#[repr(C)]
pub struct KeSymbolHandle {
    pub hold: *const KeSymbol,
}

impl KeSymbolHandle {
    pub fn get(&self) -> KeSymbolGuard {
        let h = unsafe { self.hold.as_ref_unchecked() };
        h.advance();
        KeSymbolGuard {
            v: h.ptr,
            hold: self.hold,
        }
    }
}

#[repr(C)]
pub struct KeSymbolGuard {
    v: *const (),
    hold: *const KeSymbol,
}

impl KeSymbolGuard {
    pub fn get<F>(&self) -> &F {
        if size_of::<F>() != 0 {
            panic!("Invalid downcast: size mismatch");
        }
        unsafe { (self.v as *const F).as_ref_unchecked() }
    }
}

impl Drop for KeSymbolGuard {
    fn drop(&mut self) {
        unsafe {
            self.hold
                .as_ref_unchecked()
                .punish();
        }
    }
}
