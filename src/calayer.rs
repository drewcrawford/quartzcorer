use coregraphicsr::CGFloat;
use objr::bindings::*;
use crate::CAMetalLayer;
objc_class! {
    pub struct CALayer {
        @class(CALayer)
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("contentsScale")
        @selector("setContentsScale:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl CALayer {
    pub fn contentsScale(&self, pool: &ActiveAutoreleasePool) -> CGFloat {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::contentsScale(), pool, ())
        }
    }
    pub fn setContentsScale(&mut self, scale: CGFloat, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setContentsScale_(), pool, (scale,))
        }
    }
}

impl From<&CAMetalLayer> for &CALayer {
    fn from(a: &CAMetalLayer) -> Self {
        unsafe{&* (a as *const _ as *const CALayer)}
    }
}
impl From<&mut CAMetalLayer> for &mut CALayer {
    fn from(a: &mut CAMetalLayer) -> Self {
        unsafe{&mut * (a as *mut _ as *mut CALayer)}
    }
}