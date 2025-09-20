#===============================
#SPRITE CONVERTER
#-------------------------------
#Takes any image, and samples all its pixels and returns it as a
#rs file with dependencies and a named constant array containing all the points and colors of the input
#im not masochistic enough to write a simple conversion script in rust lol
#-------------------------------

#USAGE
#Does not support partial transparency, make sure any parts you want to be transparent are set to 0 alpha
#Put resulting file in the sprites folder and declare it a public mod in mod.rs
#In the code when you want to initialize a sprite you can call Sprite::new(point, &SPRITE_NAME_HERE)

from PIL import Image;
name = input("Name of file: ")
#initializes img as the image you want to convert converted to rgba values
img = Image.open(f"{name}.png").convert("RGBA")
#pixels contains an array containing all the images rgba values
pixels = img.load()
#destructures img size with width height 
width, height = img.size
#calculates the array length by multiplying width and height
array_length = height*width
#initializes ending string
ending = ""
#iterates through entire file and appends string with the rgb values and the point they exist at
for y in range(height):
    for x in range(width):
        #destructures rgba at the current x,y
        r, g, b, a = pixels[x, y]
        #checks if alpha channel is equal to zero. if it is then remove one from the array length count
        #otherwise it appends string with current x y and rgb values.
        if a == 0:
            array_length -= 1
        else:
            ending += f"Pixel{{point:Point{{x:{x}.0,y:{y}.0}},fgcolor:NONE,bgcolor:Color::Rgb{{r:{r},g:{g},b:{b}}},string:DELETE}},"
    #prints an enter after each line 
    ending += "\n"
#initializes begining with the header that explains how to use file along with dimensions and name of file
beginning = f"""// ===============================
//  SPRITE: {name.upper()}
// -------------------------------
//  Auto-generated from {name}.png
//  Dimensions: {width}x{height}
//  Place in sprites folder and declare in mod.rs to use
//  DO NOT EDIT MANUALLY
// ===============================

use crossterm::style::Color;
use crate::shared::{{Point,Pixel,NONE,DELETE}};

pub const {name.upper()}_SPRITE: [Pixel; {array_length}] = [
"""
#removes the last comma and enter then adds the closing block
ending = ending[:-2]
finished = beginning + ending + "];"
#writes the resulting string to a file, overwrites if already exists
with open(f"{name.lower()}_sprite.rs", "w") as file:
    file.write(finished)
