// ===============================
//  SPRITES MODULE
// -------------------------------
//  Whenever you import a new sprite you need to declare it 
//  here as a pub mod in order to use it in code
// -------------------------------

use crate::shared::Size;

//DECLARE THE SPRITE YOU CREATES AS A PUB MOD HERE
//VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
//pub mod mario_sprite;
pub mod default_sprite;
pub mod blackbuck_sprite;
pub mod transparency2_sprite;
//------------------------------------------------
#[derive(Clone,Debug)]

pub struct Sprite{
    pub data:&'static [u8],
    pub size:Size,
    pub translucent_ranges:&'static [PixelRange],
    pub solid_ranges:&'static [PixelRange]

}
#[derive(Debug,Clone)]
///Contains all of the 
pub struct PixelRange{
    //line number
    pub line:usize,
    //starting index in the sprite data array
    pub src_index:usize,
    //end of the block from the source index for 
    pub src_end_index:usize,
    //position of the beginning of the block
    pub x_begin:usize,
    //offset from x
    pub x_end:usize,
}

// #[derive(Clone,Debug)]
// pub struct DynSprite{
//     pub data:Range<usize>,
//     pub size:Size,
//     pub translucent_ranges:Range<usize>,
//     pub solid_ranges:Range<usize>
// }
