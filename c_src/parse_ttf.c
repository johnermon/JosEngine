#define STB_TRUETYPE_IMPLEMENTATION
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include "stb_truetype.h"

//Loaded font Type contains a fat pointer to font data and sbtt_fontinfo handle
typedef struct{
    //pointer to font data location
    unsigned char* font_ptr;
    //size of memory allocation of font data
    size_t font_size;
    //stbtt_fonntinfo handle
    stbtt_fontinfo* font_info_ptr;
} LoadedFont;

//contains the width and height of the glyph generated from ttf file along with the a pointer to the generated black and white bm
typedef struct{
    //pointer to memory
    unsigned char *ptr;
    //size of memory allocation
    size_t size;
    //width and height of the bmp 
    int width; int height;
    //x offset and y offset of the text bmp
    int xoff; int yoff;
} LoadedGlyph;

//implements loading function in c to be called in rust program
LoadedFont c_load_font(const char* font_path, const char* font_name){
    //opens font file
    FILE *fontfile = fopen(font_path, "rb");
    //checks if font file exists and if so returns null
    if(!fontfile){
        fprintf(stderr,"Could not find font %s in fonts folder\n",font_name);
        return (LoadedFont){NULL,0,NULL};
    }
    //finds the bounds of sprite file in memory
    fseek(fontfile, 0, SEEK_END);
    long font_size = ftell(fontfile);
    //goes to beginning of file again
    fseek(fontfile,0, SEEK_SET);
    //allocates memory for the font file
    unsigned char *font_ptr = malloc(font_size);
    //if buffer is null close file and return null and free buf
    if(!font_ptr){
        fprintf(stderr,"Malloc for %s data failed\n", font_name);
        fclose(fontfile);
        free (font_ptr);
        return (LoadedFont){NULL,0,NULL};
    }
    //reads the file, saves it to buffer and outputs size for error checking
    size_t items_mem = fread(font_ptr, 1, font_size, fontfile);
    //closes fontfile
    fclose(fontfile);
    //check if the read was successful
    if (items_mem != (size_t) font_size){
        fprintf(stderr,"Read of %s was unsuccessful\n", font_name);
        //if not frees buffer and closes file while returning a null loaded font_t
        free(font_ptr);
        return (LoadedFont){NULL,0,NULL};
    }
    //creates big endian containing the header for comparisons. god pattern matching is rough in c
    uint32_t header = 
    (uint32_t){font_ptr[0]<<24}|
    (uint32_t){font_ptr[1]<<16}|
    (uint32_t){font_ptr[2]<<8}|
    (uint32_t){font_ptr[3]};
    // checks fonf file for a valid header, returns null struct to if it does not detect a valid header
    // stbtt truetype has no internal checks for validity of font file so it will produce ub if you feed
    // invalid font data. but this check will catch obvious footguns, like trying to feed it a non font file.
    if(!(((uint32_t){0x00010000} == header)||
    ((uint32_t){0x4F54544F} == header)||
    ((uint32_t){0x74727565} == header)||
    ((uint32_t){0x77746366} == header))){
        fprintf(stderr,"Font file %s has invalid header\n", font_name);
        free(font_ptr);
        return (LoadedFont){NULL,0,NULL};
    }
    //allocates new block of memory for stbtt fontinfo to live
    stbtt_fontinfo* font_info_ptr = malloc(sizeof(stbtt_fontinfo));
    //checks for malloc failure
    if (!font_info_ptr){
        fprintf(stderr, "Malloc for sbtt_font info for font %s\n", font_name);
        return (LoadedFont){NULL,0,NULL};
    }
    //inits, then if it initializes successfully return a pointer to the loaded font instance in memory
    int loaded = stbtt_InitFont(font_info_ptr, font_ptr, 0);
    if (loaded){
        return (LoadedFont){
            font_ptr,font_size,
            font_info_ptr,
        };
    }
    fprintf(stderr, "Failed to load font %s\n",font_name);
    // frees buffer and else returns null loaded font
    free(font_ptr);
    free(font_info_ptr);
    return (LoadedFont){NULL,0,NULL};
}

//called to free the memory occupied by font once font is unloaded. called by rust on drop
void c_unload_font(LoadedFont* loaded_font){
    //frees font file and stbtt handle from memory
    free(loaded_font->font_ptr);
    free(loaded_font->font_info_ptr);
}

LoadedGlyph c_generate_glyph(LoadedFont* loaded_font, int codepoint, float px){
    //initializes variables for the codepoint generation.
    int width, height, xoff, yoff;
    //uses the height in px to generate the scale factor for bmp generation
    float scale = stbtt_ScaleForPixelHeight(loaded_font->font_info_ptr, px);
    //generates bitmap
    unsigned char *glyph_ptr = stbtt_GetCodepointBitmap(
        loaded_font->font_info_ptr,
        scale,
        scale,
        codepoint,
        &width,&height,&xoff,&yoff
    );
    //calculates size of glyph bmp
    size_t size = width*height;
    //if stbtt_GetCodepointBitmap returns a null loaded glyph
    if (glyph_ptr == NULL){
        fprintf(stderr,"stbtt returned null pointer to glyph data");
        return (LoadedGlyph){0};
    }
    //if size is equal to zero return null loadedglyph
    if (size == 0){
        fprintf(stderr,"stbtt returned glyph with size zero");
        return (LoadedGlyph){0};
    }

    //returns a loaded glyph struct to rust program
    return (LoadedGlyph){
        glyph_ptr, size,
        width, height,
        xoff, yoff,
    };
}
//unloads malloc held by glyph
void c_unload_glyph(LoadedGlyph* loaded_glyph){
    //frees the memory associated with the glyph
    free(loaded_glyph->ptr);
}
//inputs 2 chars and returns an int 
inline int c_get_kerning(LoadedFont* loaded_font, int ch1, int ch2){
    //passes a ptr from the stbb_fontinfo struct into stbtt_GetCodepointKernAdvance along with both characters. generates kerning and passes it back to rust
    return stbtt_GetCodepointKernAdvance(
        loaded_font->font_info_ptr, 
        ch1, ch2
    );
}
