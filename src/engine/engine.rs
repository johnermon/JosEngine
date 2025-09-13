// ===============================
//             ENGINE.RS
// -------------------------------
//  Contains the engine. handles timing
//  fand owns input handler 
//  for camera and sprite systems.
// -------------------------------

use std::time::Instant;

use pixels::Pixels;
use crate::{

    engine::{display::Display, screens::Screen}, graphics::renderer::RenderHook, io::input::Input, shared::{Point, Size}};

pub enum EngineCommand{
    End,
    Draw,
}

pub struct Engine{
    display:Display,
    pub renderer:RenderHook,
    pub input:Input,
    last_time:Instant,
    pub tickrate:f32,
    accumulator:f32,
}

//initializes the engine
impl Engine{
    pub fn initialize(tickrate:f32, display_point:Point, screen:Box<dyn Screen>,pixels:Pixels) -> Self{
        Engine{
        //initializes display size at 1, 1 and render size 1, 1, will update on init.
        display:Display::new_at(display_point, screen, Size::is(1, 1)),
        renderer:RenderHook::create_render_hook(Size::is(1, 1),pixels),
        input:Input::new(),
        last_time:Instant::now(),
        tickrate:1.0/tickrate,
        accumulator:0.0,
        }
    }
    
    fn update_time(&mut self){
        //sets a time to be now
        let now = Instant::now();
        //sets the current frame time to be now minus the time of the last frame, clamps it to 0.25 seconds maximum
        let frame_time = (now - self.last_time).as_secs_f32().min(0.25);
        //adds frametime to the accumulator
        self.accumulator += frame_time;
        //saves now to last_time
        self.last_time = now;
    }
    
    ///Engine runes for one cycle
    pub fn run_once(&mut self) -> bool{
        //runs only if the display instance has been marked as needing 
        if self.display.needs_init{
            self.display.init_display(&self.input.input_state, &mut self.renderer);
            return false;
        }
        //updates the timing logic
        self.update_time();
        //gets input
        self.input.get_input();
        //does game logic updates until caught up with framerate
        while self.accumulator >= self.tickrate{
            //runs logic update, if the screen instance calls end of the game then tells the main loop to kill the event window loop
            if self.display.logic_update(&self.input.input_state){
                return true;
            }
            self.accumulator -= self.tickrate;
        }
        //otherwise renders the display
        self.display.render(&mut self.renderer);
        false
    }
}

