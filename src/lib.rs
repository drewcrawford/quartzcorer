mod cametallayer;
mod calayerdelegate;
mod cametaldrawable;

pub use cametaldrawable::CAMetalDrawable;
pub use cametallayer::CAMetalLayer;
pub use calayerdelegate::CALayerDelegate;

#[link(name="QuartzCore",kind="framework")] extern {}