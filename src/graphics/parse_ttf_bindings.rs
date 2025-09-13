
// ===============================
//       PARSE_TTF_BINDINGS.RS
// -------------------------------
//  anything that is either an alias for a c struct or
//  bindings for c functions go here :3
// -------------------------------
//         AN IMPORTANT NOTE
//I am aware that there are fully featured rust crates that do more or less the same thing as stbtt truetype
//I am also aware there is a 1 to 1 rust port of stbtt truetype. I am also aware that in my implementation
//there is the ability to induce undefined behavior via corrupted font files. the reason I went this route 
//was not due to a tangible performance benefit (Though stbtt truetype is blazing fast by any metric)
//but more about learning how to code c, how to bind c functions to rust, and how to work with unsafe code blocks and raw pointers.
//It was not a practical choice but rather a learning experience that, had I decided to plop in a premade rust crate I would not have gotten.
//I will implement the checksum verification when i feel ready to tackle that, at which point code will be entirely safe.

use std::{
    env::current_dir, 
    io::Error, 
    ops::Range, 
    slice::from_raw_parts
};

//C data type aliases. Just aliases but clarify what structs are doing
use std::ffi::{
    CString,
    c_float,
    c_int, 
    c_uchar, 
};
use libc::{
    c_char,
    size_t
};

use smallvec::SmallVec;

//declares external functions from parse_text.c
unsafe extern "C"{
    //private function, returns raw pointer, public facing api wraps this in the LoadedFont Struct
    unsafe fn c_load_font(font_path:*const c_char, font_name:*const c_char,) -> LoadedFont;
    unsafe fn c_generate_glyph(loaded_font:*mut LoadedFont, codepoint:c_int, px:c_float) -> LoadedGlyph;
    unsafe fn c_unload_font(loaded_font:*mut LoadedFont);
    unsafe fn c_unload_glyph(loaded_glyph:*mut LoadedGlyph);
    unsafe fn c_get_kerning(loaded_font:*mut LoadedFont,ch1:c_int,ch2:c_int) -> c_int;
}
//struct definitions for the structs from c structs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
///Opaque struct signifying the c struct stbtt_fontinfo
struct stbtt_font_info{
    _private:[u8;0],
}

#[repr(C)]
pub struct LoadedFont{
    //pointer to font data location
    pub font_ptr: *mut c_uchar,
    //size of memory allocation of font data
    pub font_size: size_t,
    //stbtt_fonntinfo handle
    font_info_ptr: *mut stbtt_font_info,
}

impl LoadedFont{
    //pulls a slice from text buffer memory and matches it to the correct ttf format
    pub fn get_format(&self) -> &str{
        //creates a slice from the first 4 bytes of the data in the ttf file which encodes format. data this is pointing to is guarenteed
        //to be valid for 2 reasons, first all the invariants regarding validity of the data are proven by the c code before being returned to rust
        //and second, this exact operation was taken in the c code successfully for the font to have loaded in the first place.
        let key:u32 = unsafe{
            ((*self.font_ptr.add(0) as u32) << 24)|
            ((*self.font_ptr.add(1) as u32) << 16)|
            ((*self.font_ptr.add(2) as u32) << 8)|
            (*self.font_ptr.add(3) as u32)
        };
        //matches key to all known valid header types supported by stbtt truetype.
        match key{
            0x00010000 => "TrueType",
            0x4F54544F => "OpenTypeCFF",
            // If you somehow get one of these loaded god bless you, god knows I tried my best do dump one from a macos9 install image
            // and the best I could do is get a version converted to standard ttf. anyways stbtt truetype supports it for some unknown
            // reason (probably because the library can be used on classic macs too) so i am keeping it here for the sake of extensiveness
            0x74727565 => "AppleTrueType",
            0x77746366 =>"TrueTypeCollection",
            _ =>"Unknown",
        }
    }
}

//deallocates the memory on LoadedFont drop
impl Drop for LoadedFont{
    fn drop(&mut self){
        if !self.font_ptr.is_null(){
            unsafe{c_unload_font(self as *mut Self);}
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadedGlyph{
    ptr: *mut c_uchar,
    pub size: usize,
    pub width:c_int, pub height:c_int,
    pub xoff:c_int, pub yoff:c_int,
}

impl LoadedGlyph{
    pub fn push_data(&self, buffer:&mut Vec<u8>){
        //extends buffer with newy created glyph slice
        buffer.extend_from_slice(
            self.as_slice()
        );
    }
    pub fn push_ranges(&self, translucent_ranges:&mut SmallVec<[Range<u8>;1024]>,solid_ranges:&mut SmallVec<[Range<u8>;1024]>){
        let glyph_slice = self.as_slice();
        todo!();
    }
    //function only used internally, returns a slice based off of 
    fn as_slice(&self) -> &[u8]{
        //unsafe sets fat pointer stored in glyph to a immutable slice in the memory. immediately after function is called,
        //glyph is dropped and drop trait deallocates the memory held in the pointer so the data this is pointing to goes
        //out of scope at the same time as the slice itself. so it is memory safe. additionally, any invariants regarding
        //null pointers possibly held in the LoadedGlyph struct are checked on return from c function for null, propagating
        //error before reaching here. so there is no chance of derefrencing null pointer.
        unsafe{
            from_raw_parts(
                self.ptr, 
                self.size
            )
        }
    }
}

impl Drop for LoadedGlyph{
    fn drop(&mut self){
        //on drop runs unload font funtion from parse_text.c
        if !self.ptr.is_null(){
            unsafe{c_unload_glyph(self as *mut Self);}
        }
    }
}

///Loads a font into memory, and returns an optional LoadedFont struct
///NOTE: Undefined behavior can be induced by trying to load a corrupted ttf file with a valid header,  (Header is verified so you cant feed it junk data)
///as sstbb_fontinfo does not enforce checksums. This is an acceptable level of memory safety to me. implementing my own
///checksum verification is on the todo list for eventually. Maybe once I get better at c.
pub fn load_font(font_name:&str) -> Option<LoadedFont>{
    //initializes new string font_path which contains the path to the font file spacified in function call
    let c_font_path= CString::new(
        //formats a new string that contains the directory 
        format!("{}/fonts/{}",current_dir().expect("Invalid directory").display(), font_name)
    ).expect("Working directory contains Null, cannot convert to cstring");
    //generates c_string from font name, unsed entirely for error formatting in the c code.
    let c_font_name = CString::new(font_name).expect("Font name contains Null, cannot convert to cstring");
    //pointer to strings only live for duration of c function call and are then dropped.
    let loaded_font = unsafe{c_load_font(c_font_path.as_ptr(),c_font_name.as_ptr())};
    //checks to see if c function returned a null pointer, if so, returns None
    if loaded_font.font_ptr.is_null(){
        return None;
    }
    Some(loaded_font)
}
///Safe rust binding for c generate glyph function. handles null invariants and propogates error if so
pub fn generate_glyph(loaded_font:&mut LoadedFont, char:char, font_height:f32) -> Result<LoadedGlyph, Error>{
    //typecasts the char as a c_int and the font height as a c_float for compatibility with c generate_glyph function
    let codepoint:c_int = char as c_int;
    let px:c_float = font_height as c_float;
    //code is safe because raw pointer to the Loaded font struct is used once to generate the glyph data and goes out of scope after func completes.
    //all invariants return a null initialized Loaded glyph struct. if rust code detects null pointer immediately propagates error
    
    let glyph = unsafe{
        c_generate_glyph(
            loaded_font as *mut LoadedFont, 
            codepoint, px
        )
    };
    //if c_generate_glyph returns a null pointer return error
    if glyph.ptr.is_null(){
        return Err(Error::other(format!("Failed to initialize codepoint for char {char}")))
    }
    Ok(glyph)
}

#[inline]
pub fn get_kerning(loaded_font:&mut LoadedFont, char1:char, char2:char) -> i32{
    //safe because only invariant for operation returns a zero for no kerning table. No invariants can result in memory unsafety 
    unsafe{
        c_get_kerning(
            loaded_font as *mut LoadedFont,
            char1 as c_int, char2 as c_int
        )
    }
}