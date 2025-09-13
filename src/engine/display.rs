// ===============================
//            DISPLAY.RS
// -------------------------------
//  Impliments all screem logic and behaviors
//  for main menu logic
// -------------------------------


use std::mem::{replace, swap};

use crate::{
    engine::screens::Screen, 
    graphics::{
        renderer::{
            Camera, 
            Pixel, 
            RenderHook
        }
    }, 
    io::input::InputState,
    object::Object,
    shared::{Point, Size, BPP}
};
use smallvec::SmallVec;


///Commands the screen can send the display instance to do things
pub enum DrawCall{
    ///Clears Display
    ClearDisplay,
    ///Moves Display
    Move(Point),
    ///Takes the key for the sprite and a point to move it to and moves it to that position
    DrawObject(usize),
    ///takes previous pixel drawcall and saves to framebuffer
    DrawPixels,
    //Passes through a vector directly to the renderer. Keep vector in screen struct, dont constantly
    //redeclare pls
    // Passthrough(Arc<Mutex<Vec<Pixel>>>)
}

pub enum LogicCall{
    ///initializes object into buffer
    InitObject(usize, Object),
    ///sets point of object in buffer
    SetObject(usize, Point),
    ///moves object relative to its current point
    MoveObject(usize, Point, f32),
    ///Draws at point with color, deletes any text
    PixelAt(Point, [u8;4]),
    ///Deletes at point
    DeleteAt(Point),
    ///Fills between two points with a static string
    FillRange(Point, Point, [u8;4]),
    ///Deletes between two points
    DelRange(Point, Point),
    ///Draws a Rectangle between two points, with color
    Rectangle(Point, Point, [u8;4]),
    ///Sets size of camera
    SetCamera(usize, Size, Point),
    ///Moves camera relative to itself
    MoveCamera(usize, Point, f32),
    ///Sets the camera that renderer will render from.
    SetOutputCamera(Option<usize>),
    ///Sets logical size of display instance. not what is displayed but the logical size of the display
    SetDisplaySize(Size),
    

}
pub struct Display{
    ///Stores point display instance is located at
    position:Point,
    ///Needs init, tells the engine to reinitialize the screen
    pub display_size:Size,
    ///Contains the current screen for the display
    pub screen:Box<dyn Screen>,
    //next screen box
    next_screen:Option<Box<dyn Screen>>,
    ///the camera currently being used to capture
    pub current_camera:Option<usize>,
    ///Needs init, tells the engine to reinitialize the screen
    pub needs_init:bool,
    ///Camera buffer stores all cameras in the scene. by default only 16 are available but you can put however many you want in
    cameras:[Camera;16],
    ///object buffer contains all the objects that the screen might write to
    pub objects:Vec<Object>,
    objects_max_index:usize,
    ///display batches obect draw then executes it all at once
    pub object_draw_buffer:SmallVec<[usize;512]>,
    ///Stores a buffer containing all display draw commands
    pub draw_buffer:SmallVec<[DrawCall;256]>,
    ///Stores a buffer for all engine logical commands
    pub logic_buffer:SmallVec<[LogicCall;256]>,
    ///Display buffer stores all the pixels on the display field
    pub display_buffer:Vec<u8>,
    ///Pixel buffer holds pixel draw commands, for direct screen drawing or perhaps procedural effects
    pub pixel_buffer:SmallVec<[Pixel;1024]>,

}
impl Display{
    ///Constructor function for a screen.
    pub fn new_at(position:Point, screen:Box<dyn Screen>, display_size:Size) -> Self{
        Display{
            //position of the screen. doesnt really do much right now
            position,
            display_size,
            //screen fields, contain the current screen being displayed and a buffer for the next screen to be displayed
            screen,
            next_screen:None,
            current_camera:None,
            needs_init:true,
            //the size of the smallvec stack allocated array for both draw and logic calls is set to 256 but preallocates another 256
            //in the heap so you dont have to dynamically resize if for whatever reason you go over 256 draw or logic calls in one update
            cameras:[Camera::default();16],
            objects:vec![Object::default();2048],
            objects_max_index:0,
            object_draw_buffer:SmallVec::with_capacity(2048),
            draw_buffer:SmallVec::with_capacity(2048),
            logic_buffer:SmallVec::with_capacity(2048),
            display_buffer:Vec::with_capacity(display_size.pixels()*BPP),
            pixel_buffer:SmallVec::with_capacity(2048),
        }
    
    }
    ///initializes the display instance with a certain screen, called on screen change
    pub fn init_display(&mut self, input_state:&InputState, renderer:&mut RenderHook){
        //clears the display buffer for the display instance
            self.draw_buffer.clear();
            //filld renderer frame buffer with zeros
            self.display_buffer.clear();
            self.display_buffer.resize(self.display_size.pixels()*BPP,0);
            //clears pixel buffer
            self.pixel_buffer.clear();
            //sets screen to next screen
            if let Some(next_screen) = self.next_screen.take() {
                let old = replace(&mut self.screen, next_screen);
                drop(old);
            }
            //runs the init screen command for loaded screen
            self.screen.init_screen(&mut self.draw_buffer, &mut self.logic_buffer);
            //updateds display logic and render logic
            self.logic_update(input_state);
            //renders screen
            self.render(renderer);
            //sets display needs init to false
            self.needs_init = false;
    }

    /// runs the game logic, handles screen updates and then matches each logic call with code to be executed.
    pub fn logic_update(&mut self , input_state:&InputState) -> bool{
        //clears the draw buffer in case frame was skipped and other screen updates are currently contained, makes sure renderer
        //only processes newest draw calls
        self.draw_buffer.clear();
        //destructures the return screen and kill from the screen update function
        let (return_screen, kill) = self.screen.update(&mut self.logic_buffer, input_state);
        //if the screen decides to kill the game return true to the engine to exit window
        if kill{return true;}
        //if screen returns a screen then save to displays next screen field and flags the display for update.
        if return_screen.is_some(){
            //sets the screen field in the display instance to whatever the last screen returned
            self.next_screen = return_screen;
            //sets the needs update flag to true so next update the engine initializes the new screen
            self.needs_init = true;
        }
        //drains logic call commands in the logic buffer
        for command in self.logic_buffer.drain(..){
            match command{
                LogicCall::InitObject(key, object) =>{
                    //sets the object at index key in the object buffer to an object instance
                    self.objects[key] = object;
                    if key > self.objects_max_index{
                        self.objects_max_index = key;
                    }
                }
                LogicCall::SetObject(key, point) =>{
                    let object = &mut self.objects[key];
                    let Point{mut x,mut y} = point;
                    //moves the position of the object in the object buffer by changing its point
                    x = x.clamp(0.0,(self.display_size.width.saturating_sub(object.sprite.size.width)) as f32);
                    y = y.clamp(0.0,(self.display_size.height.saturating_sub(object.sprite.size.height)) as f32);
                    object.point = Point::at(x,y);
                    //pushes object to the drawbuffer
                    //self.draw_buffer.push(DrawCall::DrawObject(key));
                }
                LogicCall::MoveObject(key,dir, amount) =>{
                    //sets object to a mutable reference to the object buffer at index key
                    let object = &mut self.objects[key];
                    //gets a point from the objec nudged by the direction and the amount
                    let Point{mut x, mut y} = object.point.nudge(dir, amount);
                    //clamps x and y to only be able to move in bounds and saturating sub to avoid crashing due to trying to index outside of the bounds
                    x = x.clamp(0.0,(self.display_size.width.saturating_sub(object.sprite.size.width)) as f32);
                    y = y.clamp(0.0,(self.display_size.height.saturating_sub(object.sprite.size.height)) as f32);
                    //sets objects point to x and y 
                    object.point = Point::at(x,y);
                    //pushes object to the drawbuffer
                    //self.draw_buffer.push(DrawCall::DrawObject(key));
                }
                LogicCall::SetCamera(key,size,point) => {
                    //sets camera to be a mutable reference to cameras at index key (clamped to a maximum of 15 for the max index of the camera array)
                    let camera= &mut self.cameras[key.min(15)];
                    if camera.camera_size <= self.display_size{
                        camera.camera_size = size;
                    }else{
                        eprintln!("Tried to set camera larger than display size!")
                    }
                    let Point{mut x,mut y} = point;
                    //moves the position of the object in the object buffer by changing its point
                    x = x.clamp(0.0,(self.display_size.width.saturating_sub(camera.camera_size.width)) as f32);
                    y = y.clamp(0.0,(self.display_size.height.saturating_sub(camera.camera_size.height)) as f32);
                    camera.point = Point::at(x,y);
                }
                LogicCall::MoveCamera(key,dir,amount)=>{
                    let Point{mut x, mut y} = self.cameras[key.min(15)].point.nudge(dir, amount);
                    let camera = &mut self.cameras[key.min(15)];
                    //clamps camera to only be able to move in bounds to avoid crashing
                    x = x.clamp(0.0,(self.display_size.width.saturating_sub(camera.camera_size.width)) as f32);
                    y = y.clamp(0.0,(self.display_size.height.saturating_sub(camera.camera_size.height)) as f32);
                    camera.point = Point::at(x,y);
                }
                LogicCall::SetOutputCamera(key) =>{
                    //sets the current output camera to the index of the camera array at key
                    if let Some(key) = key{
                        self.current_camera = Some(key.min(15));
                    }else{
                        self.current_camera = None;
                    }
                }
                LogicCall::SetDisplaySize(size)=>{
                    //sets display size, this is not the render size but the size in in game units of how large the display is.
                    self.display_size = size;
                    self.display_buffer.resize(size.pixels()*BPP, 0);
                }
                LogicCall::PixelAt(mut point,color) =>{
                    point = point + self.position;
                    self.pixel_buffer.push(Pixel::with_color(point, color));
                }
                LogicCall::DeleteAt(mut point)=>{
                    point = point + self.position;
                    self.pixel_buffer.push(Pixel{point, color:[0,0,0,255], translucent:false});
                }
                LogicCall::FillRange(mut point,mut point2,color)=>{
                    point = point + self.position;
                    point2 = point2 + self.position;
                    fill_range(&mut self.pixel_buffer, point, point2, color);
                }
                LogicCall::DelRange(mut point,mut point2 )=>{
                    point = point + self.position;
                    point2 = point2 + self.position;
                    fill_range(&mut self.pixel_buffer, point, point2, [0,0,0,255]);
                }
                LogicCall::Rectangle(mut point,mut point2,color)=>{
                    point = point + self.position;
                    point2 = point2 + self.position;
                    draw_rect(&mut self.pixel_buffer, point, point2, color);
                }
            }
        }
        false
    }
    // pub fn redraw(&mut self){
    //     //lets object contain a slice of all objects in buffer
    //     let objects = &self.object_draw_buffer[..self.objects_max_index];
    //     //iterates through objects and enumerates results
    //     for (id, object) in objects.iter().enumerate(){
    //         //iterates again through buffer
    //         for other in objects[..id].iter().enumerate(){
    //             //skips object if it is at same index ads object it is currently checking
    //             if other.0 == id{continue;}
    //             //if object contains other object, add both to object draw buffer to be redrawn
    //             if self.object_draw_buffer[].contains(&other.1){
    //                 self.object_draw_buffer.extend([
    //                     id,
    //                     other.0
    //                 ]);
    //             }
    //         }
    //     }
    // }
    ///matches each display command to the corresponding function and executes it
    pub fn render(&mut self, renderer:&mut RenderHook){
        //runs screens draw command
        self.screen.draw(&mut self.draw_buffer);
        //drains the command buffer and matches the command to the proper renderer commands
        for command in self.draw_buffer.drain(..){
            match command{
                DrawCall::ClearDisplay =>{
                    self.display_buffer.fill(0);
                }
                DrawCall::Move(point) =>{
                    self.position = self.position + point;
                }
                DrawCall::DrawObject(id) => {
                     if self.objects[id].sprite.size <= self.display_size{
                        self.object_draw_buffer.push(id)
                    }else{
                        eprintln!("Tried to draw object larger than display size!")
                    }
                }
                DrawCall::DrawPixels =>{
                    renderer.buffer_pixels(
                        &mut self.display_buffer[..self.display_size.pixels()*BPP],
                        self.display_size,
                        &mut self.pixel_buffer);
                }

            };
        }
        
        //iters through object draw buffer and draws all objects that are marked with a dirty flag
        for object_id in self.object_draw_buffer.drain(..){
            let object = &mut self.objects[object_id];
            if object.needs_draw{
                renderer.buffer_object(&object, &mut self.display_buffer[..self.display_size.pixels()*BPP],&self.display_size);
            }
            //object.needs_draw = false;
        }
        if let Some(camera) = self.current_camera{
            //checks if camera size is the same as pixels framebuffer size, if not changes it.
            let output_camera= self.cameras[camera];
            if output_camera.camera_size != renderer.render_size{
                renderer.change_size(output_camera.camera_size);
            }
            //camera captures the portion of the display buffer it is looking at
            renderer.capture(&mut self.display_buffer[..self.display_size.pixels()*BPP], &self.display_size, &output_camera);
        //if no camera is active copy entire display to pixel buffer.
        }else{
            //checks if render size is the same as display size and updates it if not
            if self.display_size != renderer.render_size{
                renderer.change_size(self.display_size);
            }
            //copys entire render buffer to pixels back buffer
            renderer.copy_buffer(&self.display_buffer);
        }
    }
}

//helper functions for the draw calls, as to not clutter up the main match case
fn fill_range(buf:&mut SmallVec<[Pixel;1024]>, mut point:Point, mut point2:Point,color:[u8;4]){
    //Swaps the x and y of the 2 points such that x and y are always both lower on point 1 than point 2
    if point.x > point2.x{
        swap(&mut point.x,&mut point2.x);
    }
    if point.y > point2.y{
        swap(&mut point.y, &mut point2.y);
    }
    //Scans through all of the positions in the bound of x and y and saves to buffer
    for x in point.x as i32 ..= point2.x as i32{
        for y in point.y as i32 ..= point2.y as i32{
            buf.push(Pixel::with_color(Point::at(x as f32,y as f32), color));
        }
    }
}

fn draw_rect(buf:&mut SmallVec<[Pixel;1024]>, mut point:Point, mut point2:Point,color:[u8;4]){
    //Swaps the x and y of the 2 points such that x and y are always both lower on point 1 than point 2
    if point.x > point2.x{
        swap(&mut point.x,&mut point2.x);
    }
    if point.y > point2.y{
        swap(&mut point.y, &mut point2.y);
    }
    //Pushes horizontal lines to the buffer
    for x in point.x as i32..=point2.x as i32{
        buf.push(Pixel::with_color(Point::at(x as f32,point.y),color));
        buf.push(Pixel::with_color(Point::at(x as f32,point2.y),color));
    }
    //Pushes vertical lines to the buffer
    for y in point.y as i32+1..point2.y as i32{
        buf.push(Pixel::with_color(Point::at(point.x,y as f32),color));
        buf.push(Pixel::with_color(Point::at(point2.x,y as f32),color));
    }
}