use gilrs::{Button, Event, EventType, GamepadId, Gilrs};
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::shared::*;

pub struct Input{
    pub keyboard_input:WinitInputHelper,
    gamepad_input:Gilrs,
    gamepad_id:Option<GamepadId>,
    pub input_state:InputState
}
pub struct InputState{
    pub up:bool,
    pub down:bool,
    pub left:bool,
    pub right:bool,
    pub start:bool,
    pub select:bool,
    pub north:bool,
    pub south:bool,
    pub west:bool,
    pub east:bool,
    pub direction:Point
}
//creates input handler
impl Input{
    pub fn new() -> Self{
        //creates default input struct
        let mut input =Input{
            keyboard_input:WinitInputHelper::new(),
            gamepad_input:Gilrs::new().expect("Failed to initialize gilrs"),
            gamepad_id:None,
            input_state:InputState{
                up:false,
                down:false,
                left:false,
                right:false,
                start:false,
                select:false,
                north:false,
                south:false,
                west:false,
                east:false,
                direction:Point::at(0.0,0.0)
            }
        };
        //if gilrs detects gamepad save it to gamepad_id field of Input struct, if not remains none
        if let Some(gamepad_id) = input.gamepad_input.gamepads().next(){
            input.gamepad_id = Some(gamepad_id.0);
        }
        input
    }

    pub fn get_input(&mut self){

        //dumps the input for Gilrs so it can be read by get_input. also auto connects/ reconnects controller.
        while let Some(Event{id,event,..}) = self.gamepad_input.next_event(){
            match event{
                EventType::Connected => self.gamepad_id = Some(id),
                EventType::Disconnected => if self.gamepad_id == Some(id){self.gamepad_id = None;},
                _ =>{}
            }
        }
    
        //Checks keyboard inputs
        self.input_state.up = self.keyboard_input.key_held(KeyCode::ArrowUp);
        self.input_state.down = self.keyboard_input.key_held(KeyCode::ArrowDown);
        self.input_state.left = self.keyboard_input.key_held(KeyCode::ArrowLeft);
        self.input_state.right = self.keyboard_input.key_held(KeyCode::ArrowRight);
        self.input_state.start = self.keyboard_input.key_held(KeyCode::Enter);
        self.input_state.select = self.keyboard_input.key_held(KeyCode::Escape);
        self.input_state.north = self.keyboard_input.key_held(KeyCode::Tab);
        self.input_state.south = self.keyboard_input.key_held(KeyCode::Space);
        self.input_state.west = self.keyboard_input.key_held(KeyCode::KeyE);
        self.input_state.east = self.keyboard_input.key_held(KeyCode::ShiftRight);
        //gets gamepad input and ors it with keyboard input
        if let Some(gamepad_id) = self.gamepad_id{
            self.input_state.up |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::DPadUp);
            self.input_state.down |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::DPadDown);
            self.input_state.left |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::DPadLeft);
            self.input_state.right |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::DPadRight);
            self.input_state.start |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::Start);
            self.input_state.select |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::Select);
            self.input_state.north |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::North);
            self.input_state.south |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::South);
            self.input_state.west |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::West);
            self.input_state.east |= self.gamepad_input.gamepad(gamepad_id).is_pressed(Button::East);
        }
        //defines a new tuple with all directional inputs as members
        let directions = (
            self.input_state.up,
            self.input_state.down,
            self.input_state.left,
            self.input_state.right,
        );
        //translates directional inputs into a motion vector.
        self.input_state.direction = match directions{
            (true, false, false, false)=> UP,
            (false, true, false, false)=> DOWN,
            (false, false, true, false)=> LEFT,
            (false, false, false, true)=> RIGHT,
            (true, false, true, false)=> UP_LEFT,
            (true, false, false, true)=> UP_RIGHT,
            (false, true, true, false)=> DOWN_LEFT,
            (false, true, false, true)=> DOWN_RIGHT,
            _=> Point::at(0.0, 0.0)
        }
    }
}
