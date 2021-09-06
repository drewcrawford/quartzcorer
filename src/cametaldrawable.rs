use objr::bindings::*;
use metalr::MTLTexture;
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
    pub fn texture(&mut self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLTexture> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::texture(), pool, () );
            MTLTexture::assume_nonnil(ptr).assume_retained()
        }
    }
    pub fn present(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::present(), pool, ()) }
    }
}

