use objr::bindings::*;
use metalr::{MTLTexture,MTLDrawable};
// use metal::*;

objc_instance! {
    pub struct CAMetalDrawable;
}
objc_selector_group! {
    pub trait CAMetalDrawableSelectors {
        @selector("present")
        @selector("texture")
    }
    impl CAMetalDrawableSelectors for Sel {}
}

impl CAMetalDrawable {
    pub fn texture(&self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLTexture> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::texture(), pool, () );
            MTLTexture::assume_nonnil(ptr).assume_retained()
        }
    }
    pub fn present(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::present(), pool, ()) }
    }
    pub fn as_metal_drawable(&self) -> &MTLDrawable {
        //safe because CAMetalDrawable inherits from MTLDrawable
        unsafe { self.cast() }
    }
}

