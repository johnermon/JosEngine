
    
    
    use std::ops::{Add, Div, Mul, Sub};
    //used for simplification, a reasonable approximation of sqrt2
    pub const SQRT_2:f32 = 0.70710677;

    pub const BPP:usize = 4; //BPP is bytes per pixel, currently its 4 bytes per pixel.


    //Directional constants, useful anywhere you need to nudge points. 
    pub const UP:Point = Point{x:0.0, y:-1.0};
    pub const UP_LEFT:Point = Point{x:-SQRT_2, y:-SQRT_2};
    pub const UP_RIGHT:Point = Point{x:SQRT_2, y:-SQRT_2};
    pub const DOWN:Point = Point{x:0.0, y:1.0};
    pub const DOWN_LEFT:Point = Point{x:-SQRT_2, y:SQRT_2};
    pub const DOWN_RIGHT:Point = Point{x:SQRT_2, y:SQRT_2};
    pub const LEFT:Point = Point{x:-1.0, y:0.0};
    pub const RIGHT:Point = Point{x:1.0, y:0.0};

#[derive(Clone, Copy,Default, Debug, PartialEq, PartialOrd)]
pub struct Size{
    pub width:usize,
    pub height:usize,
    
}

//impliments the pixels method for the struct size
impl Size{
    pub fn is(width:usize, height:usize) -> Self{
        Size {
            width,
            height
        }
    }
    pub fn pixels(&self) -> usize{
        (self.height * self.width) as usize
    }
    pub fn change(&mut self, width:usize, height:usize){
        self.width = width;
        self.height = height;
    }
}   

    #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
    ///Point stores an x, y position on the terminal grid, can also be encoded with netagives to work as a directional offset
    pub struct Point{pub x:f32, pub y:f32}


    impl Add for Point{
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Point{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl Add<Size> for Point{
        type Output = Self;
        fn add(self, rhs: Size) -> Self::Output {
            Point{
                x: self.x + rhs.width as f32,
                y: self.y + rhs.height as f32,
            }
        }
    }
    impl Sub for Point{
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Point{
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    impl Sub<Size> for Point{
        type Output = Self;
        fn sub(self, rhs: Size) -> Self::Output {
            Point{
                x: self.x - rhs.width as f32,
                y: self.y - rhs.height as f32,
            }
        }
    }
        impl Mul<Point> for Point{
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Point{
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }
        impl Mul<f32> for Point{
        type Output = Self;
        fn mul(self, rhs: f32) -> Self::Output {
            Point{
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }
    impl Div for Point{
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Point{
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }
    //impliment block for point 
    impl Point{
        ///Returns new point with coordinates x, y
        #[inline(always)]
        pub fn at(x:f32,y:f32) -> Self{
            Point{x,y}
        }
        ///Takes in an array and a point and outputs a nudged point by that array
        pub fn nudge(&self, dir:Point,amount:f32) -> Point{
            return *self + dir*amount;
        }
        ///Nudges self by a certain amount
        pub fn nudge_self(&mut self, dir:Point, amount:f32){
            *self = self.nudge(dir,amount)
        }
    }