use makepad_render::*;
mod buttonlogic;
pub use crate::buttonlogic::*;
mod normalbutton;
pub use crate::normalbutton::*;

pub fn live_register(cx:&mut Cx){
    println!("REGGIN");
    crate::normalbutton::live_register(cx);
}
