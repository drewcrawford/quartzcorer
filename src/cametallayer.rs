use objr::bindings::*;
use metalr::{MTLDevice, MTLPixelFormat};
use crate::calayerdelegate::CALayerDelegate;
use crate::CAMetalDrawable;
use coregraphicsr::*;

objc_class! {
    pub struct CAMetalLayer {
        @class(CAMetalLayer)
    }
}

objc_selector_group! {
    trait CAMetalLayerSelectors {
        @selector("setDevice:")
        @selector("setDelegate:")
        @selector("nextDrawable")
        @selector("setBackgroundColor:")
        @selector("setPixelFormat:")
        @selector("setFramebufferOnly:")
        @selector("setWantsExtendedDynamicRangeContent:")
        @selector("drawableSize")
        @selector("setDrawableSize:")
    }
    impl CAMetalLayerSelectors for Sel {}
}

#[allow(non_snake_case)]
impl CAMetalLayer {
    //todo: Move this onto a CALayer type
    //(CALayer is not yet implemented)
    pub fn setBackgroundColor(&mut self, color: Option<&CGColorRef>, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setBackgroundColor_(), pool, (color.as_ptr(),))
        }
    }
    pub fn setFramebufferOnly(&mut self, value: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setFramebufferOnly_(), pool, (value,))
        }
    }
    pub fn setWantsExtendedDynamicRangeContent(&mut self,  value: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setWantsExtendedDynamicRangeContent_(), pool, (value,))
        }
    }
    pub fn setDevice(&mut self, device: Option<&MTLDevice>,  pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setDevice_(), pool, (device.as_ptr(),))
        }
    }
    ///Unsafe because unsupported pixel formats raise an objc exception which is UB
    pub unsafe fn setPixelFormat(&mut self,  pixel_format: MTLPixelFormat, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setPixelFormat_(), pool, (pixel_format.field(),))
    }
    pub fn setDelegate(&mut self,  delegate:Option<&CALayerDelegate>, pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::setDelegate_(), pool, (delegate.as_ptr(),)) }
    }
    pub fn nextDrawable(&mut self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<CAMetalDrawable>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::nextDrawable(), pool, ());
            CAMetalDrawable::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn drawableSize(&self, pool: &ActiveAutoreleasePool) -> CGSize {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::drawableSize(), pool, ())
        }
    }
    pub fn setDrawableSize(&mut self, size: CGSize, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setDrawableSize_(), pool, (size,))
        }
    }
}

#[test] fn test() {
    autoreleasepool(|pool| {
        //note that most CI doesn't support this, but I don't know how critical that is.
        // let device = MTLDevice::default().unwrap();

        let mut layer = unsafe{ CAMetalLayer::class().alloc_init(pool).assume_mut() };
        // layer.setDevice( Some(&device), pool);
        unsafe{ layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm, pool) }

        let obj = NSObject::class().alloc_init(pool);
        let as_delegate = unsafe{ obj.cast() };
        layer.setDelegate( Some(&as_delegate), pool);

        let size = CGSize{width: 100.0, height: 200.0};
        layer.setDrawableSize(size, pool);
        let returned_size = layer.drawableSize(pool);
        assert_eq!(size,returned_size);
    })
}