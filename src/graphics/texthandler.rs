// ===============================
//             TEXTPARSER.RS
// -------------------------------
//  interacts with c libraries for image parsing and wraps them in safe functions for 
//  use in the rest of the code
// -------------------------------

use std::{
    alloc::{alloc, Layout}, mem::ManuallyDrop, ops::Range, ptr::{copy_nonoverlapping, NonNull}, slice::from_raw_parts
};

use crate::graphics::parse_ttf_bindings::{generate_glyph, LoadedFont, LoadedGlyph};

use josie_collections::josie_vec::{josievec_extend::ExtendType, JosieVec};

pub(crate) struct GlyphRange{
    line:usize,
    range:Range<usize>
}

pub(crate) struct SizedGlyph<'a>{
    char:char,
    transparent_indexes:&'a [Range<usize>],
    solid_indexes:&'a [Range<usize>],
}

impl<'a> Default for SizedGlyph<'a>{
    fn default() -> Self {
        Self { char: Default::default(), transparent_indexes: Default::default(), solid_indexes: Default::default() }
    }
}

impl<'a> SizedGlyph<'a>{
    pub fn push_ranges(&mut self,font_index:usize ,atlas:&mut JosieVec<u8>, loaded_glyph:&LoadedGlyph){
        unsafe{atlas.bulk_extend_guarded(ExtendType::Ammortized, |start_ptr,end_ptr|{
            //creates working josievecs
            let working_transparent_ranges:JosieVec<GlyphRange> = JosieVec::with_capacity(loaded_glyph.size);
            let working_solid_ranges:JosieVec<GlyphRange> = JosieVec::with_capacity(loaded_glyph.size);
            //the most disgusting typecast i think i have ever done lol, needed to 
            let start = &mut *(start_ptr as *mut *mut u8 as *mut *mut GlyphRange);
            //creates a new slice contaning all the sprite data
            let glyph_slice = from_raw_parts(loaded_glyph.ptr, loaded_glyph.size);
            let iter = glyph_slice.iter().peekable();
            for element in iter{
            }
        })};
    }
}

pub(crate) struct FontAtlas{
    ptr:NonNull<u8>,
    len:usize
}
pub struct SizedFont<'a>{
    atlas:JosieVec<u8>,
    glyphs:&'a [SizedGlyph<'a>]

}
impl<'a> Default for SizedFont<'a>{
    fn default() -> Self {
        Self { atlas: Default::default(), glyphs: Default::default() }
    }
}

pub struct Font<'a>{
    font_handle:LoadedFont,
    sized_fonts:Vec<SizedFont<'a>>
}

pub fn load_font<'a>(loaded_font:LoadedFont) -> Font<'a>{
    todo!();
}

pub fn create_sized_font<'a>(loaded_font:&mut LoadedFont, font_size:f32, charset:&[char]) -> SizedFont<'a>{
    //creates defauld uninitialized sized font struct
    let mut sized_font = SizedFont::default();
    //working storage for the generated glyphs, dropped and contents copied to end of bump at the end of sized font generation
    let mut working_glyphs:JosieVec<SizedGlyph<'a>> = JosieVec::with_capacity(charset.len());
    //reserves capacity in the font atlas for worst case scenerio, note vec is only being used here as storage for the contents, drop code will treat like u8 which is fine because nothing that will be stored here will do heap allocations
    sized_font.atlas.reserve_exact(9*(font_size as usize)^2 + charset.len()*4);
    //iterates through the charset
    for char in charset.iter().map(|char|char.clone()){
        let glyph = generate_glyph(loaded_font, char, font_size).expect(&format!("faileds to generate glyph {}", char));
        unsafe{
            //copies the content of the memory allocation to the point on the vector
            copy_nonoverlapping(glyph.ptr, sized_font.atlas.as_mut_ptr().add(sized_font.atlas.len()), glyph.size);
            //new len is the  len after the copy_nonoverlapping
            let new_len = sized_font.atlas.len() + glyph.size;
            //current range of the loaded sprite in the josievec bump
            let curr_index = sized_font.atlas.len();
            //manually sets length to new size
            sized_font.atlas.set_len(new_len);
            //creates new sized glyph struct
            let mut sized_glyph = SizedGlyph::default();
            //pushes translucent and solid ranges to the new sized_glyph
            sized_glyph.push_ranges(curr_index,&mut sized_font.atlas,&glyph);
            working_glyphs.push(sized_glyph);

        }
    }
    unsafe{
        let glyphs_ptr = sized_font.atlas.as_mut_ptr().add(sized_font.atlas.len()) as *mut SizedGlyph<'a>;
        copy_nonoverlapping(
            //source is the working glyphs pointer
            working_glyphs.as_mut_ptr(), 
            //destination is the end of the atlas
            glyphs_ptr,
            //the number of working glyphs created
            working_glyphs.len()
        );
        //creates a slice from raw parts
        sized_font.glyphs = from_raw_parts(glyphs_ptr, working_glyphs.len());
    }
    sized_font.atlas.shrink_to_fit();

    sized_font
    
}


// use std::{
//     collections::HashMap, f32::MAX, io::Error, ops::Range
// };

// use bumpalo::Bump;
// use smallvec::SmallVec;

// use crate::{
//     graphics::{parse_ttf_bindings::*, renderer::Renderable},
//     shared::{Point, Size},
// };

// impl Renderable for String{
//     fn render(&self, point:Point){
//         todo!();
//     }
// }

// ///Sets the default preallocation for the hash map of fonts. 
// const FONTS_PREALLOC:usize = 64;
// ///Sets the preallocated vec size for different font sizes per font struct. if you use more than 16 sizes of the same font
// ///nudge this up to avoid reallocations.
// const FONT_SIZE_PREALLOC:usize = 16;

// ///Preallocates on the stack 1024kb worth of space for computing font values smallvec
// const MAX_FONT_SMALLVEC_PREALLOC:usize = 1048576;

// ///FontCode is the tuple struct in which is used as the key for the hashmap. It contains a string for font name and usize for size of font
// pub type FontCode = String;
// ///Char sprite is a tuple struct that contains a dynsprite for 
// pub struct CharCode{
//     pub char:char,
//     pub code:u32,
// }

// #[derive(Debug)]
// pub struct GlyphSprite<'a>{
//     pub size:Size,
//     pub offset:Point,
//     pub translucent_ranges:&'a [TranslucentRange],
//     pub solid_ranges:&'a [SolidRange],
// }

// impl<'a> GlyphSprite<'a>{
//     ///constructor for GlyphSprite struct.
//     pub fn new(glyph:&LoadedGlyph,index:usize,translucent_index:usize,solid_index:usize) -> Self{
//         //creates new size struct with the width and height held inside of 
//         let glyph_size = Size::is(glyph.width as usize,glyph.height as usize);
//         //creates new point for the offsett of the glyph based
//         let glyph_offset = Point::at(glyph.xoff as f32, glyph.yoff as f32);
//         let data_slice:&'a [u8] = [index..index+glyph_size.pixels()];
//         Self{
//             //Range starts in the index, which is the current length of the data vector in parent font struct and ends
//             //in the index plus number of pixels 
//             size:glyph_size,
//             offset:glyph_offset,
//             translucent_ranges,
//             solid_ranges,
//         }
//     }
// }



// pub type FontAtlas = &'a [u8];

// pub struct PixelIndex<'a>{
//     //index of raw sprite data
//     index:&'a[u8],
//     //line number
//     pub line:usize,
//     //position of the beginning of the block
//     pub x_begin:usize,
//     //offset from x
//     pub x_end:usize,
// }

// pub type TranslucentIndex = PixelIndex;

// pub type SolidIndex = PixelIndex;

// pub type TranslucentRange =  &'a [TranslucentIndex];

// pub type SolidRange = &'a [SolidIndex];

// pub type TranslucentRanges = &'a [TranslucentRange];

// pub type SolidRanges = &'a [SolidRange];

// pub struct SizedFont<'a>{
//     ///contains a vector of the struct CharSprite
//     pub data:Bump,
//     pub font_atlas:&'a [u8],
//     pub metadata:&'a [GlyphSprite<'a>],
// }

// impl<'a> SizedFont<'a>{
//     fn new_sized_font(&mut self, loaded_font:&mut LoadedFont ,charset:&[()], font_size:f32) -> Result<(),Error>{
//         //capactity is max size of the atlas given every character generates a bitmap exatly the size of the font size every time
//         //at the end of sized font generation it is shrunk to fit the actual capacity and then bumpaloed into an area of memory
//         //contiguous with the range indexes for improved cache performance on text rasterization. :3
//         let capacity:usize = font_size as usize^2 * charset.len();
//         //sets working atlas to be a vector with capacity capacity 
//         let working_font_atlas:Vec<u8> = Vec::with_capacity(capacity);
//         //both working ranges have a stack allocated size equal to 1024(way more than enough for pretty much every font)
//         //but are both heap allocated to worst case scenerio (font where every pixel alternates between translucent and solid)
//         let working_translucent_ranges:SmallVec<[Range<u8>;1024]> = SmallVec::with_capacity(capacity/2);
//         let working_solid_ranges:SmallVec<[Range<u8>;1024]> = SmallVec::with_capacity(capacity/2);
        
//         //iterates throughout the charset and generates a glyph for each char. then uses that Loaded glyph to generate a new GlyphSprite
//         //and 
//         for &char in charset{
//             //grabs the glyph from its codepoint in the ttf file and returns a Loaded glyph struct
//             let glyph = generate_glyph(loaded_font, char, font_size)?;
//             glyph.push_data(&mut working_font_atlas);
//             glyph.push_ranges(&mut working_translucent_ranges, &mut working_solid_ranges);
//         }
//         //calculates size of bump based off of the size of the current buffers
//         let bump_capacity =
//             working_font_atlas.len()*size_of::<&[()]>() +
//             working_translucent_ranges.len()*size_of::<&[()]>() +
//             working_solid_ranges.len()
//         ;
//         //allocates bump on the heap first to make sure that when the temporary working vecs are dropped it is not fragmented.
//         //bump capacity is equal to 2 times the capacity so it could hypothetically take all the elements of hypothetically maxed
//         //out sprites
//         self.data = Bump::with_capacity(bump_capacity);
//         //iterates through working_font_atlas and saves each value to the location in bump, returning a reference to the data
//         self.font_atlas = self.data.alloc_slice_fill_iter(working_font_atlas.iter());
//         //drops working_font_atlas
//         drop(working_font_atlas);
//         let final_translucent_ranges =
        
//         SizedFont{
//             metadata: working_font_atlas.into_boxed_slice(),

//         };
//         Ok(())
//     }
// }

// ///Font is a struct that contains a vec data that contains all the font data and a vec of charsprite structs that slice the data vec
// /// contains a boxed slice 
// pub struct Font{
//     ///contains a vector of sized fonts for each of the font sizes intitialized for this specific font
//     pub font_sizes:Vec<SizedFont>,
//     pub charset:Box<[char]>,
//     loaded_font:LoadedFont,
// }

// impl Font{

//     fn new_font(&mut self, font_name:String, charset:&[char], font_sizes:&[f32]) -> Result<(), Error>{
//         //loads ttf into memory based off of string, returns error if font doesnt exist or font file is corrupted
//         let mut loaded_font = load_font(&font_name)?;
//         //iterates through charset and generates a glyph for each char provided and inserts 
        
//         //shrinks working font data to fit the size of the buffer
//         working_font.data.shrink_to_fit();
//         //inserts completed font into hashmap
//         self.loaded_fonts.insert(FontCode(font_name, font_height), working_font);
//         Ok(())
//     }

//     ///public constructor for font class, creates a font with all fields pre-allocated ready to be written
//     pub fn generate_font_size(size:usize, font_height:usize, charset:&[char]) -> Self{
//         //max_px is the maximum size of the buffer assuming every character is exactly height squared in size.
//         //it is shrunk to actual size once font initialization is over. done to avoid reallocations during text generation
//         let max_px = size*font_height^2;
//         Font{
//             metadata:Vec::with_capacity(size),
//             data:Vec::with_capacity(max_px),
//             charset:charset.to_vec().into_boxed_slice()
//         }
//     }
// }

// ///Text handler is the public facing text handling struct. It contains the hash map of all the fonts currently loaded into memory at every size
// pub struct TextHandler{
//     pub loaded_fonts:HashMap<FontCode, Font>,
// }

// impl TextHandler{
//     pub fn init() -> Self{
//         TextHandler{
//             loaded_fonts:HashMap::with_capacity(FONTS_PREALLOC)
//         }
//     }

    
// }

// fn slices_from_ranges(){}

