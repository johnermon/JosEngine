// ===============================
//            SNAKE.RS
// -------------------------------
//  Yes, I know linked lists are an inefficient way to code snake.
//  No, I don't care -\('.')/-
// -------------------------------

//declare crates to import

use rand::{rng, Rng};
use smallvec::SmallVec;
use crate::{engine::{display::{DrawCall, LogicCall}, screens::{main_menu::MainMenu, Screen}}, io::input::InputState, shared::*};

//Initializes Constants

//constant speed decides how many frames before game updates
const SPEED:u8 =3;

//BOARD_SIZE decides the size of board
const BOARD_SIZE:f32 = 20.0;

//Sets characters and colors for the thiongs in the game.
//By default snake apple and wall are aliases of DELETE constant from shared, due to default behavior
//for them being to just change the background color
//if you want any to have a custom character you can change it here.
//you will additionally have to change the color field from background color to foreground color in order to color the string

const SNAKE_COLOR:[u8;4] = [175, 105, 238, 255];

const WALL_COLOR:[u8;4] = [0, 0, 255, 127];

const APPLE_COLOR:[u8;4] = [167, 199, 231, 255];

const BACKGROUND_COLOR:[u8;4] = [248,185,212, 255];
//define structs

///Apple stores a Point and two bools, one signifying whether apple has been eaten, one for if it needs to be drawn
struct Apple{
    coord:Point, 
    eaten:bool,
}

///SnakeBody is a linked list with one Point and a boxed optional instance of itself.
struct SnakeBody{
    coord:Point,
    segment:Option<Box<SnakeBody>>,
}

///Snake stores dir, and next_dir as arrays, the next coord, and current head position, 
/// optional coordinates of last tail, and an optional SnakeBody
pub struct Snake{
    dir:Point,
    next_dir:Point,
    coord:Point,
    next_coord:Point,
    last_tail:Point,
    segment:Option<Box<SnakeBody>>,
    apple:Apple,
    score:u16,
    frame_counter:u8,
}

//Implements the screen trait for snake.
impl Screen for Snake{
    ///Initializes screen by drawing border, drawing the game 
    
    fn init_screen(&self, drawbuffer: &mut SmallVec<[DrawCall;256]>,logic_buffer:&mut SmallVec<[LogicCall;256]>){
        //initializes all variables for the display.
        logic_buffer.extend([
            //Draws Snake
            LogicCall::PixelAt(self.coord, SNAKE_COLOR),
            //Draws Apple
            LogicCall::PixelAt(self.apple.coord, APPLE_COLOR),
            //Creates Wall
            LogicCall::FillRange(Point::at(0.0,0.0), Point::at(BOARD_SIZE+1.0, BOARD_SIZE+1.0), BACKGROUND_COLOR),
            //creates background
            LogicCall::Rectangle(Point::at(0.0,0.0), Point::at(BOARD_SIZE+1.0,BOARD_SIZE+1.0), WALL_COLOR),
            //disables output camera
            LogicCall::SetOutputCamera(None),
            LogicCall::SetDisplaySize(Size::is(BOARD_SIZE as usize + 2,BOARD_SIZE as usize + 2)),
        ]);
        //draws initial elements
        drawbuffer.extend([
            //Sends all pixel draw commands to the frame buffer
            DrawCall::DrawObject(1),

        ]);
    }

    fn update(&mut self,logic_buffer:&mut SmallVec<[LogicCall;256]>, input_state:&InputState) -> (Option<Box<dyn Screen>>,bool){
        logic_buffer.extend([
            //Deletes last tail
            LogicCall::PixelAt(self.last_tail, BACKGROUND_COLOR),
            //Draw snake coord
            LogicCall::PixelAt(self.coord,SNAKE_COLOR),
            //Draw apple
            LogicCall::PixelAt(self.apple.coord, APPLE_COLOR),

        ]);
        //changes direction
        self.change_dir(input_state);
        if self.frame_counter == SPEED{
            //Updates snake direction by copying the value in next_dir if the values are different
            if self.dir != self.next_dir{self.dir = self.next_dir}
            //Runs collision_logic and if it outputs true then breaks the loop
            if self.handle_collision(){
                //Returns main menu to the display instance
                return (Some(MainMenu::init()),false);
            }
            //Matches apple.eaten. Appends snake and spawns new apple if true, moves forward if not
            if self.apple.eaten{
                self.eat_apple_and_grow();
            }else{
                self.move_forward();
            }
            self.frame_counter = 0;
        }else{
            self.frame_counter +=1;
        }
        //returns no new screen to the display instance and doesnt kill the loop.
        (None,false)
    }

    ///Redraws Snake head, snake tail and apple if it has been respawned.
    fn draw(&self, drawbuffer: &mut SmallVec<[DrawCall;256]>){
        //initializes buffer for display commands
        drawbuffer.extend([
            //Sends all pixel draw commands to the frame buffer
            DrawCall::DrawPixels,
        ]);
    }
}
//Implement methods for Snake
impl Snake{
    ///public constructor for snake.
    pub fn init() -> Box<dyn Screen> {
        let mut snake = Snake{
            dir:RIGHT,
            next_dir:RIGHT,
            coord:Point::at(BOARD_SIZE/2.0, BOARD_SIZE/2.0),
            //next_coord is just one point to the right of coord
            next_coord:Point::at(BOARD_SIZE/2.0+1.0,BOARD_SIZE/2.0),
            last_tail:Point::at(BOARD_SIZE/2.0-1.0, BOARD_SIZE/2.0),
            segment:None,
            //apple is temporary just for the sake of declaring the struct
            apple:Apple{
            coord:Point::at(0.0,0.0),
            eaten:false,
            },
            score:0,
            frame_counter:0,
        };
        snake.new_apple();
        Box::new(snake)
    }
    ///Creates a new apple and checks for collisions with snake. repeats until it no collision
    fn new_apple(&mut self){
        //Initializes random number generator
        let mut rng = rng();
        //Loop is active until apple is created and doesnt collide with snake
        loop{
            //Initializes apple
            let apple = Apple{
            coord:Point{
                //randomly selects x and y values for apple
                x:rng.random_range(1..=BOARD_SIZE as i32)as f32,
                y:rng.random_range(1..=BOARD_SIZE as i32)as f32,
            },
            eaten:false,
            };
            //Checks if apple collides with the snake, restarts loop if so
            if self.check_collision(apple.coord){
                continue;
            }
            self.apple = apple;
            break;
        }
    }

    ///Reads keyboard input and based on arrow key pressed changes the next_dir enum in snake struct.
    ///Guardrail against moving into itself. updates next_coord
    fn change_dir(&mut self,input_state:&InputState){
        //reads the keyboard once every TICK_TIME/2 milliseconds
            if input_state.up && self.dir != DOWN {self.next_dir = UP};
            if input_state.down && self.dir != UP {self.next_dir = DOWN};
            if input_state.left && self.dir != RIGHT {self.next_dir = LEFT};
            if input_state.right && self.dir  != LEFT {self.next_dir = RIGHT};
            //Nudges the next coord with next_dir
            self.next_coord = self.coord.nudge(self.next_dir, 1.0);
    }

    ///Extends snake by taking previous head, turning it into a body segment and putting in new snake object
    fn append_snake(&mut self){
        //old_body takes ownership of the entirity of snakes body
        let old_body = self.segment.take();
        let body= SnakeBody{
            //snakes current coord becomes next body segments coord and the old body becomes the old body.
            coord:self.coord,
            segment:old_body
        };
        //Point moved is made the new coord and body is boxed and set to segment
        self.coord = self.next_coord;
        self.segment = Some(Box::new(body));
    }

    ///Moves snake via calling append_snake() and removing the last entry of the linked list.
    /// Also takes previous tail and saves to head struct for rendering purposes.
    fn move_forward(&mut self){
        self.append_snake();
        //Finds last node in linked list and removes it
        let mut current = &mut self.segment;
        loop{
            match current{
                Some(node) if node.segment.is_none() =>{
                    //Saves last tail to the snake struct
                    self.last_tail = node.coord;
                    //Remove the last node, deleting tail
                    *current = None;
                    break;
                }
                Some(node) =>{
                    //Move to the next node
                    current = &mut node.segment;
                }
                None =>{
                    self.last_tail = self.coord.nudge(self.dir,1.0);

                    let last_tail_nudge = self.dir;
                    self.dir = self.dir*-1.0;
                    self.last_tail.nudge(last_tail_nudge, 1.0);
                },
            }
        }
    }

    ///Takes a point and returns true if it collides with any segment of the snake
    fn check_collision(&self, point: Point) -> bool{
        //Checks the head of the snake against point
        if self.coord == point{
            return true;
        }
        //Iterates through the rest of the list and checks every other segment against other point
        let mut current = &self.segment;
        while let Some(node) = current{
            if node.coord == point{
                //Returns true if point collides with snake
                return true;
            }else{
                //Move to next node
                current = &node.segment;
            }    
        }
        //returns false if no collisions
        false
    }

    //Checks whether or not snake has hit wall
    fn check_collision_wall(&self) -> bool{
        //checks if head is out of bounds and returns bool, by default bounds are 1-16 in both x and y directions
        let Point{x,y} = self.next_coord;
        x > BOARD_SIZE as f32|| x == 0.0 || y > BOARD_SIZE as f32|| y == 0.0
    }

    ///Handles all the collision, returns true on collision with self or wall and sets apple to eaten if collides with apple
    fn handle_collision(& mut self) -> bool{
    if (self.check_collision(self.next_coord)||self.check_collision_wall()) &&
        self.coord != self.last_tail{
        //returns true if snake collides with either wall or self
        return true;
    }else if self.next_coord == self.apple.coord{
        //sets apple to eaten if snake collides with apple
        self.apple.eaten = true;
    }
    false
    }

    ///Eats apple and grows
    fn eat_apple_and_grow(&mut self){
        self.score += 1;
        self.append_snake();
        self.new_apple();
    }
}