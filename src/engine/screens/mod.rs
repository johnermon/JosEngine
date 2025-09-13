// ===============================
//             SCREEN MOD
// -------------------------------
//  Contains all the the implimentation 
//  for screen trait, and declares all screens
// -------------------------------

use smallvec::SmallVec;

use crate::{engine::display::{DrawCall, LogicCall}, io::input::InputState};


pub mod snake;
pub mod main_menu;

///Screen trait impliments all the methods for an individual scene to be able to be run by the display struct
pub trait Screen{
    ///Initializes The screen
    fn init_screen(&self, drawbuffer:&mut SmallVec<[DrawCall;256]>, logic_buffer:&mut SmallVec<[LogicCall;256]>);
    ///Updates any logic and returns the next screen once the screen is finished, or a boolean that kills the main loop.
    fn update(&mut self, logic_buffer:&mut SmallVec<[LogicCall;256]>,input:&InputState) -> (Option<Box<dyn Screen>>,bool);
    ///Draws the current screen state
    fn draw(&self,drawbuffer:&mut SmallVec<[DrawCall;256]>);
}