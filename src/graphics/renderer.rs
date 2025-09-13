// ===============================
//             RENDERER.RS
// -------------------------------
//  Contains all the the code that interracts
//  with pixels. contains blit methods and
//  alpha transparency methods
// -------------------------------

use crate::object::Object;
use crate::shared::*;
use pixels::Pixels;
use smallvec::SmallVec;
///Camera is a dumb box that contains a position and a size, referenced by the capture function.
#[derive(Clone, Copy, Debug)]
pub struct Camera{
    pub point:Point,
    pub camera_size:Size,
}
//default camera, used to populat camera buffer in display instance
impl Camera{
    pub fn default() -> Self{
        Camera{
            camera_size:Size::is(1, 1),
            point:Point::at(0.0,0.0),
        }
    }
    
}

///Pixel is a pixel draw command. it is sent to the renderer, it contains a color value a point and a flag for whether or not it is translucent or solid.
#[derive(Clone, Debug)]
pub struct Pixel{
    pub point:Point,
    pub color:[u8;4],
    pub translucent:bool
}
impl Pixel{
    pub fn with_color(point:Point, color:[u8;4]) -> Self{
        if color[3] != 255{
            let [mut r,mut g,mut b,a] = color;
            r = (fast_divide(r as u16*a as u16)) as u8;
            g = (fast_divide(g as u16*a as u16)) as u8;
            b = (fast_divide(b as u16*a as u16)) as u8;
            Pixel{
                point,
                color:[r,g,b,a],
                translucent:true
            }
        }else{
            Pixel{
                point:point,
                color,
                translucent:false
            }
        }
    }
}

pub trait Renderable{
    fn render(&self, point:Point);
}
///held by engine and borrowed mutably during rendering renderhook owns the instance of pixels created at the initilaization of the program
pub struct RenderHook{
    pub render_size:Size,
    pub pixels:Pixels,
}

impl RenderHook{
    ///Public constructor for RenderHook
    pub fn create_render_hook(render_size:Size, pixels:Pixels)-> Self{
        RenderHook{
            render_size,
            pixels,
        }
        
    }
    ///Changes size of pixels buffer and cameras internal resolution it uses for calculations
    pub fn change_size(&mut self, size:Size){
        self.render_size = size;
        self.pixels.resize_buffer(size.width as u32, size.height as u32).unwrap();
    }

    ///Draws pixel Draw command to display buffer
    pub fn buffer_pixels(&mut self,display_buffer:&mut [u8], display_size:Size,pixel_buffer: &mut SmallVec<[Pixel;1024]>){
        //drains pixel buffer
        for pixel in pixel_buffer.drain(..){
            //clamps x and y to the display size
            let x = pixel.point.x.floor().clamp(0.0,display_size.width as f32) as usize;
            let y = pixel.point.y.floor().clamp(0.0,display_size.height as f32) as usize;
            //calculates the index for each pixel
            let index = x*BPP + y*display_size.width*BPP;
            if pixel.translucent{
                composite(&mut display_buffer[index..=index+3], &pixel.color);
            }else{
                //if they arent copy them directly
                display_buffer[index..index+BPP].copy_from_slice(&pixel.color);
            }
        }
    }
    ///Draws object to display buffer
    pub fn buffer_object(&mut self, object:&Object,display_buffer:&mut [u8], render_size:&Size){
        
        //sets point to  x and y and converts to usize, sets width to sprites width
        //clamps to 0.0 to avoid any underflow cases in the case of negative points
        //upper bounds check unnecessary, in the final render call frame vector is sliced
        //to exact size before being copied to pixels backbuffer.
        let x = object.point.x.floor() as usize;
        let y = object.point.y.floor() as usize;
        //calculates the index to draw the sprite at in the screen vector
        let y_bytes = render_size.width*BPP;
        let index: usize= x*BPP + y*render_size.width*BPP;
        //FIRST PASS - SOLID BLOCKS
        //reads out all the ranges of static pixels in the sprites static ranges section and draws them on the screen
        for range in object.sprite.solid_ranges{
            //finds the position in the screen buffer to place the slice of the sprite based of parameters
            display_buffer[index + y_bytes*range.line + range.x_begin..(index + y_bytes*range.line + range.x_end)]
            //copys the corresponding slice of the sprite data.
            .copy_from_slice(&object.sprite.data[range.src_index..range.src_end_index]);
        }
        //SECOND PASS - TRANSLUCENT BLOCKS
        for range in object.sprite.translucent_ranges{
            let source = &object.sprite.data[range.src_index..range.src_end_index];
            //finds the position in the screen buffer to place the slice of th sprite based of parameters
            let destination = &mut display_buffer[index + y_bytes*range.line + range.x_begin..(index + y_bytes*range.line + range.x_end)];
            //iterates through the pixel array and the destination and composites the source onto the destination
            for (dst, src) in destination.chunks_exact_mut(BPP).zip(source.chunks_exact(BPP)){
                composite(dst, src);
            }
        }
    }
    
    pub fn capture(&mut self, display_buffer:&[u8], display_size:&Size, camera:&Camera){
        //cameras point to  x and y and converts to usize, sets width cameras width
        //clamps to 0.0 to avoid any underflow cases in the case of negative points
        let x = camera.point.x.floor() as usize;
        let y = camera.point.y.floor() as usize;
        let width = display_size.width*BPP;
        let camera_width = camera.camera_size.width * BPP;
        let camera_height = camera.camera_size.height;
        //calculates the index to grab the line from on the display buffer
        let index= x * BPP;
        //chunks the display buffer into lines. skipping to the y value and only taking as many chunks as the camera height
        let lines = display_buffer.chunks_exact(width).skip(y).take(camera_height);
        for (row,line) in lines.enumerate(){
            //copys the data in the line to the indexed point in the framebuffer
            self.pixels.frame_mut()[row*camera_width..row*camera_width + camera_width].copy_from_slice(&line[index..index + camera_width]);
        }
    }

    ///copys buffer directly to pixel buffer, used for copying entire display buffer to pixels buffer
    pub fn copy_buffer(&mut self, frame:&[u8]){
        //copies data in frame buffer into the pixels backbuffer
        self.pixels.frame_mut().copy_from_slice(&frame[..self.render_size.width * self.render_size.height * BPP]);
    }
}

fn composite(dst:&mut [u8],src:&[u8]){
    let composite_alpha = 255 - src[3] as u16;
    //r
    dst[0] = src[0] + (fast_divide(dst[0] as u16*composite_alpha)) as u8;
    //g
    dst[1] = src[1] + (fast_divide(dst[1] as u16*composite_alpha)) as u8;
    //b
    dst[2] = src[2] + (fast_divide(dst[2] as u16*composite_alpha)) as u8;
    //a
    dst[3] = src[3] + (fast_divide(dst[3] as u16*composite_alpha)) as u8;
}

//fast divide helper function for alpha compositing, doesnt actually divide anything but gets the exact same number as true division by 255 would
fn fast_divide(value:u16) -> u8{
    (((value + 128) + ((value + 128) >> 8)) >> 8) as u8
}