#===============================
#SPRITE CONVERTER
#-------------------------------
#Takes any image, and samples all its pixels and returns it as a
#rs file with dependencies and a named constant array containing all the u8 values in the image,
#along with arrays corresponding to indexes of transparent and solid pixels for efficient rendering
#-------------------------------

#USAGE
#Put resulting file in the sprites folder and declare it a public mod in mod.rs.

#In the code when you want to initialize a sprite you can insert the sprite instance directly into the object you are attaching to or
#if not attached to object than just pass it into the renderer.

#not the most elegent script ever, no doubt could be way nicer and but hey, making quick script is what python is for amirite gamers?
#im also not masochistic enough to write a simple conversion script in rust lol

from PIL import Image
#gets input
filename = input("Name of file: ")
#load and convert to RGBA
img = Image.open(f"{filename}").convert("RGBA")
#creates new variable name that is filename with everything past the extenttion cut off.
name = filename[:filename.find(".")]
#destructures width and height 
width, height = img.size
#increases width by one
width += 1
#create a new image 1 wider, filled with transparent pixels. needed for algorithm that finds blocks of translucent and solid pixels
new_img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
#paste the original image
new_img.paste(img, (0, 0))
#replace the original with the padded one
img = new_img
#pixels contains all the images rgba values
pixels = img.load()
#initializes all variables and lists that will be used in script
solid = ["\n"]
translucent = ["\n"]
parts = []
NONE, TRANSLUCENT, SOLID, TRANSPARENT = 0, 1 , 2, 3
translucent_count = 0
solid_count = 0
#iterates through every pixel in the image and saves the indexes for contiguous blocks of translucent and solid pixels
for y in range(height):
    current , last = NONE , NONE
    for x in range(width):
        #destructures rgba at the current x,y
        r, g, b, a = pixels[x, y]
        #if alpha is in the translucent set current to translucent
        if 0 < a < 255:
            current = TRANSLUCENT
        #if color is solid set current to solid
        elif a == 255:
            current = SOLID
        #otherwise set current to transparent
        else:
            current = TRANSPARENT
        #if current is different from last (changed from solid to translucent or vice-versa) save the generated ranges to
        #the requisite list and incriment the counter for each type in that list
        if current != last:
            if last == TRANSLUCENT:
                translucent_count += 1
                translucent.append(f"[{y},{y*((width-1)*4) + min_slice*4},{(y*((width-1)*4) + x*4)}, {min_slice*4},{x*4}],")
            elif last == SOLID:
                solid_count += 1
                solid.append(f"[{y},{y*((width-1)*4) + min_slice*4},{(y*((width-1)*4) + x*4)}, {min_slice*4},{x*4}],")
            #reset slice for next iteration
            min_slice = x
        #sets the last to current
        last = current
    #adds an enter to all 3 lists for the rows
    solid.append("\n")
    translucent.append("\n")

for y in range(height):
    for x in range(width-1):
        #destructures rgba at the current x,y
        r, g, b, a = pixels[x, y]
        if a == 0:
            parts.append("0,0,0,0,")
        elif a == 255:
            parts.append(f"{r},{g},{b},{a},")
        else:
            #otherwise it appends string with current x y and premultiplied rgb values.
            parts.append(f"{(r*a + 127)//255},{(g*a + 127)//255},{(b*a + 127)//255},{a},")

    #prints an enter after each line 
    parts.append("\n")
    #initializes ending string
#creates ending by joining the entire list and removing the final enter and comma
ending = ''.join(parts).rstrip(",\n")

#initializes begining with the header that explains how to use file along with dimensions and name of file
beginning = f"""// ===============================
//  {name.upper()}_SPRITE.RS
// -------------------------------
//  generated from {filename}
//  dimensions: {width -1 }x{height}
//  place in sprites folder and declare in sprites mod.rs to use
//  DO NOT EDIT MANUALLY
// -------------------------------

use crate::{{graphics::sprites::Sprite, shared::Size}};

pub const {name.upper()}_SPRITE:Sprite = Sprite{{data:&{name.upper()}_SPRITE_DATA,size: Size{{width:{width-1},height:{height}}},
translucent_ranges:&{name.upper()}_TRANSLUCENT, solid_ranges:&{name.upper()}_SOLID}};

const {name.upper()}_SPRITE_DATA:[u8;{(width -1)*height*4}]= [{ending}];

const {name.upper()}_TRANSLUCENT:[[usize;5];{translucent_count}]= [{''.join(translucent)}];

const {name.upper()}_SOLID:[[usize;5];{solid_count}]= [{''.join(solid)}];
"""
#writes the resulting string to a file, overwrites if already exists
with open(f"{name.lower()}_sprite.rs", "w") as file:
    file.write(beginning)
