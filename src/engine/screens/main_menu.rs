// ===============================
//             MAIN_MENU.RS
// -------------------------------
//  Contains all the the implimentation 
//  for main menu logic. currently just a test
//  for camera and sprite systems.
// -------------------------------

use smallvec::SmallVec;

use crate::{
    engine::{display::{DrawCall, LogicCall}, screens::{snake::Snake, Screen}}, graphics::sprites::{blackbuck_sprite::BLACKBUCK_SPRITE, transparency2_sprite::TRANSPARENCY2_SPRITE}, io::input::InputState, object::Object, shared::{Point, Size}};
pub struct MainMenu{
}


impl Screen for MainMenu{
    fn init_screen(&self,drawbuffer:&mut SmallVec<[DrawCall;256]>,logic_buffer:&mut SmallVec<[LogicCall;256]>){
        logic_buffer.extend([
            LogicCall::InitObject(1,Object::new(Point::at(0.0,0.0), TRANSPARENCY2_SPRITE)),
            //sets output camera to camera 0
            LogicCall::SetOutputCamera(Some(0)),
            //initializes object set to background into position 1 on the 
            LogicCall::InitObject(0, Object::new(Point::at(0.0,0.0), BLACKBUCK_SPRITE)),
            //sets display size to 300 300
            LogicCall::SetDisplaySize(Size::is(512,512)),
            //sets camera 0 to size 50 at point 0
            LogicCall::SetCamera(0, Size::is(100, 100), Point::at(0.0,0.0)),
        ]);
        //drawcall to draw object 1 (main menu screen)
        drawbuffer.extend([
                DrawCall::DrawObject(0),
                DrawCall::DrawObject(1),
                
            
        ]);
    }
            
    fn update(&mut self, logic_buffer:&mut SmallVec<[LogicCall;256]>  ,input_state:&InputState) -> (Option<Box<dyn Screen>>,bool){
            //pushes the camera by the current input direction
            logic_buffer.push(LogicCall::MoveObject(1, input_state.direction, 5.0));

            //if shift or east button on controller is pressed then initialize and switch to camera to camera 1 
            if input_state.east{
                logic_buffer.extend([
                    LogicCall::SetOutputCamera(None),
                    LogicCall::SetCamera(1, Size::is(512,512), Point::at(0.0,0.0))
                    ]);
            }
            //return new instance of snake to the display instance if start or enter is pressed
            if input_state.start {return (Some(Snake::init()),false);}
            //if escape or select is pressed quit game
            if input_state.select {return (None,true);}
            //otherwise dont change screen state and dont kill the program
        (None, false)
    }
    //draws screen
    fn draw(&self, drawbuffer:&mut SmallVec<[DrawCall;256]>){
        drawbuffer.extend([
                DrawCall::DrawObject(0),
                DrawCall::DrawObject(1),
                
            ]);
    }
}

    impl MainMenu{
        //public constructor for main menu
        pub fn init() -> Box<dyn Screen>{
            Box::new(MainMenu{})
            }
    
    }
