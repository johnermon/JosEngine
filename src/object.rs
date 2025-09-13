use crate::{graphics::sprites::{default_sprite::DEFAULT_SPRITE, Sprite}, shared::Point};

#[derive(Clone, Debug)]
pub struct Object{
        pub sprite:Sprite,
        pub point:Point,
        pub needs_draw:bool,
    }

impl Object{
    #[inline(always)]
    pub fn new(point:Point, sprite:Sprite) -> Self{
        Object{
            sprite,
            point,
            needs_draw:true
        }
    }
    pub fn default() -> Self{
        Self{
            sprite:DEFAULT_SPRITE,
            point:Point::at(0.0,0.0),
            needs_draw:false,
        }
    }
    pub fn bounds(&self) -> Point{
        self.point + self.sprite.size
    }
    pub fn bounds_neg(&self) ->Point{
        self.point - self.sprite.size
    }
    pub fn contains(&self, other:&Object) -> bool{
    if self.point <= other.point && other.point <= self.bounds()||
    self.point <= other.bounds() && other.bounds() <= self.bounds(){
        true
    }
    else{
        false
    }
    }
}
