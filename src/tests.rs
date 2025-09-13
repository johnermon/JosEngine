use crate::graphics::parse_ttf_bindings::{load_font};

///tries loading different fonts and checking the results for invariants.
pub fn test_font_load(){
    //initializes boxed slice of of &str names for fonts to test loading. not an array because a small heap allocation is okay for tests if i just want to be able
    //to add new fonts by just declaring them and not changing the length of an array.
    let fonts_to_test:Box<[&str]> = vec![
        "nonexistent_font.ttf",
        "invalid_font.ttf",
        "ttf_test.ttf",
        "otf_test.otf",
        "legacy_macos_test.ttf"
    ].into_boxed_slice();
    //iterates through each font name, tries to load it, and if successful tells you the format type for the font
    for font_name in fonts_to_test{
        //tries to load font and if font load is successful prints font format
        eprintln!("Trying to load font {font_name}");
        if let Some(loaded_font) = load_font(font_name){
            eprintln!("Success!\nLoaded font format is {}",loaded_font.get_format());
        }
    }
}