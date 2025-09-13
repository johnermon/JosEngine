// ===============================
//          JOSENGINE
// -------------------------------
//  "Lua is a language even script kiddies like you can understand"
//  – something someone once told me (making me learn Rust out of spite)
// -------------------------------

//Imports
use josengine::{
    engine::{
        engine::Engine, 
        screens::main_menu::MainMenu
    }, 
    shared::Point, 
    tests::test_font_load
};

use pixels::{wgpu::Maintain, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{
        Event,
        WindowEvent
    },
    event_loop::{
        ControlFlow, 
        EventLoop
    }, 
    window::WindowBuilder
};
//selects mimalloc for linux and windows versions of the build but uses default allocator for mac builds.
#[cfg(any(target_os = "windows", target_os = "linux"))]
use mimalloc::MiMalloc;
#[cfg(any(target_os = "windows", target_os = "linux"))]
#[global_allocator]
static ALLOC:MiMalloc = MiMalloc;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
//number of updates per second
const UPDATE_RATE:f32 = 60.0;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    test_font_load();
    run_program()
}

fn run_program() -> Result<(), Box<dyn std::error::Error>>{
    //initializes winit event loop and window
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("JOSEngine")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .expect("WindowBuilder::build failed"
    );

    //initializes pixels surface texture with the windows inner size
    let window_size = window.inner_size();
    //initializes engine with default parameters, updated on first run once command
    let mut engine = Engine::initialize(
        UPDATE_RATE,
    Point::at(0.0,0.0),
        MainMenu::init(), 
        Pixels::new(1, 1, 
            SurfaceTexture::new(
            window_size.width.max(1), 
            window_size.height.max(1), &window)) 
            .expect("error")
        );

    //no matter how hard i tried i couldnt understand how to do do this myself... its just the winit init the rest of the code is mine
    event_loop.run(move |event, elwt|{
    //sets control flow mode to poll
    elwt.set_control_flow(ControlFlow::Poll);
    match &event{
        //handles window events for the window
        Event::WindowEvent { window_id, event:window_event } if window_id == &window.id() =>{
            //updates the keyboard input struct on the input handler with newest keyboard data
            engine.input.keyboard_input.update(&event); 
            match window_event{
                //closes window on close requested
                WindowEvent::CloseRequested =>{
                    elwt.exit();
                }
                WindowEvent::Resized(size) =>{
                    //resizes pixels surface texture based on window logical size
                    let _ = engine.renderer.pixels.resize_surface(size.width, size.height);
                }
                WindowEvent::RedrawRequested =>{
                    //renders pixels backbuffer
                    let _ = engine.renderer.pixels.render();
                    engine.renderer.pixels.device().poll(Maintain::Poll);
                }
                WindowEvent::Focused(new_focused_state) =>{
                    //on window focus hides cursor
                    window.set_cursor_visible(!*new_focused_state);
                    
                }
                _=>{}
            }
        }
        Event::AboutToWait =>{
            //runs engine for one cycle, exits if engine returns true
            engine.run_once().then(||elwt.exit());
            //requests window to redraw
            window.request_redraw();
        
        }
        _ =>{}
        }
    })?;
Ok(())
}