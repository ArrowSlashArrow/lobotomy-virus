use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, source::Source};

use winapi;
use winapi::ctypes::c_int;
// keystates that activate each lobotomy
const KEYSTATES: [u16; 12] = [
    0x41, // Air detected
    0x57, // Water on the hill
    0x46, // Fire in the hole
    0x43, // area Confirmed
    0x52, // Rock on the ground
    0x49, // wInd from the landscape
    0x4C, // Lightning on the road
    0x48, // bees in the Hive
    0x4B, // Kids at the basement
    0x4D, // Magma in the bound
    0x42, // Blood in the bath
    0x53 // Shadows from the grave
];

const KEYSTRINGS: [&str; 12] = [
    "Air detected",
    "Water on the hill",
    "Fire in the hole",
    "Area confirmed",
    "Rock on the ground",
    "Wind from the landscape",
    "Lightning on the road",
    "Bees in the hive",
    "Kids at the basement",
    "Magma in the bound",
    "Blood in the bath",
    "Shadows from the grave"
];

// for whatever reason i couldnt get other fiel paths to work and i dont know how to get them to work so im gonna leave it as this and figure it out later
const SOUNDFILES: [&str; 12] = [
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/air_detected.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/water_on_the_hill.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/fire_in_the_hole.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/area_confirmed.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/rock_on_the_ground.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/wind_from_the_landscape.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/lightning_on_the_road.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/bees_in_the_hive.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/kids_at_the_basement.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/magma_in_the_bound.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/blood_in_the_bath.wav",
    "C:/Users/anton/RustroverProjects/i_love_gd_cologne/sfx/shadows_from_the_grave.wav"
];



// self explanatory
fn virus(keystates: [u16; 12], prev_keystates: [u16; 12], stream_handle: &OutputStreamHandle) {
    for k in 0..12 {
        if keystates[k] >= 1 && prev_keystates[k] < 1 { // this is here to make sure that holding the key doesnt crash your audio driver
            println!("{}", KEYSTRINGS[k]); // TODO: replace with framebuffer write event
            play_sound(SOUNDFILES[k], stream_handle);
        }
    }
    return;
}
// once again, self explanatory
fn play_sound(filepath: &str, stream_handle: &OutputStreamHandle) {
    let file = BufReader::new(File::open(filepath).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("error playing sound");
}


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // each second theres a 1 in 10,000 chance of a lobotomy
    // TODO: display dialog box "oh no, you just got a lobotomy, and now your head's been fucked up!" with the fire in the hole icon

    let mut prev_keystates: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // previous keystates
    let mut lobotomy_counts: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // unused statistics (todo: occasionally a window pops up with these stats displayed with the message "lobotomy update!")
    // println!("Hello, world!"); // line is here to make sure the program runs
    loop {
        // runs every time it feels like it (idk how often that is but its often enough)
        let mut current_keystates: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        unsafe {
            for key in 0..12 {
                current_keystates[key] = winapi::um::winuser::GetAsyncKeyState(KEYSTATES[key] as c_int) as u16;
            }
        }
        virus(current_keystates, prev_keystates, &stream_handle);
        prev_keystates = current_keystates;
    }
}

// yes im a geometry dash player lol
