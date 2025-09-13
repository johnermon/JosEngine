// ===============================
//          BUILD
// -------------------------------
//  declares c dependencies during build
//  i could have done everything c code does
//  with native rust crates but learning to do this is real important
// -------------------------------
use cc::Build;
use std::fs::read_dir;

fn main(){
    let mut build = Build::new();
    //checks every c file in the c directory and adds it to the cc builder
    for entry in read_dir("c_src").unwrap(){
        //
        let path = entry.unwrap().path();
        //checks if the file is a c file and then if it is adds the file to the build path
        if path.extension().and_then(|s| s.to_str()) == Some("c"){
            build.file(path);
        }
    }
    build.include("c").compile("img_processing");

    //tells cargo to only rebuild if changed
    println!("cargo:rerun-if-changed=c");
    
}