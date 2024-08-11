#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use winapi::um::winuser::{MB_ICONINFORMATION, MB_OK, LR_LOADFROMFILE, LR_SHARED};
use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;
use rand::Rng;
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

// o noes i doxxed myself
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

fn plural(singular: &str) -> &str {
    return match singular {
        "Air detected" => "Airs detected",
        "Water on the hill" => "Waters on the hills",
        "Fire in the hole" => "Fires in the hole",
        "Area confirmed" => "Areas confirmed",
        "Rock on the ground" => "Rocks on the ground",
        "Wind from the landscape" => "Winds from the landscape",
        "Lightning on the road" => "Lightnings on the road",
        "Bees in the hive" => "Bees in the hive",
        "Kids at the basement" => "Kids at the basement",
        "Magma in the bound" => "Magmas in the bound",
        "Blood in the bath" => "Bloods in the bath",
        "Shadows from the grave" => "Shadows from the grave",
        _ => "distinct lobotomies"
    };
}


// self explanatory
fn virus(keystates: [u16; 12], prev_keystates: [u16; 12], stream_handle: &OutputStreamHandle) -> [u16; 12] {
    let mut lobotomy_count_diff: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for k in 0..12 {
        if keystates[k] >= 1 && prev_keystates[k] < 1 { // this is here to make sure that holding the key doesnt crash your audio driver
            // println!("{}", KEYSTRINGS[k]); // TODO: replace with framebuffer write event -> isnt done but im shutting it up
            play_sound(SOUNDFILES[k], stream_handle);
            lobotomy_count_diff[k] += 1;
        }
    }
    return lobotomy_count_diff;
}
// once again, self explanatory
fn play_sound(filepath: &str, stream_handle: &OutputStreamHandle) {
    let file = BufReader::new(File::open(filepath).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("error playing sound");
}


// for the life of me i cannot get this to workm,

unsafe fn lobotomy_messagebox(message: &str, title: &str) {
    let icon_path = CString::new(r"C:\Users\anton\RustroverProjects\i_love_gd_cologne\src\fire_in_the_hole.ico").unwrap();

    let icon_handle = winapi::um::winuser::LoadImageA(
        std::ptr::null_mut(),   // hInstance (none)
        icon_path.as_ptr() as *const i8, // Path to the icon file
        winapi::um::winuser::IMAGE_ICON,  // Resource type
        0,  // Default width
        0,  // Default height
        LR_LOADFROMFILE | LR_SHARED
    );

    if icon_handle.is_null() {
        let error_code = winapi::um::errhandlingapi::GetLastError();
        // println!("Error loading icon, err code: {}", error_code); // this is always a 0 (i have no clue what i going on here)
    }

    // it is also important to note that i tried to use winapi::um::winuser::MessageBoxIndirectA but i couldnt figure out how to get the icon to work
    // so im just using a default info icon

    winapi::um::winuser::MessageBoxA(
        std::ptr::null_mut(),
        CString::new(message).unwrap().as_ptr() as *const i8,
        CString::new(title).unwrap().as_ptr() as *const i8,
        MB_OK | MB_ICONINFORMATION
    );
}
unsafe fn init_lobotomy() {
    lobotomy_messagebox("the lobotomy was successful! now you will start see some colourful things :)", "Fire in the hole!");
}

unsafe fn lobotomy_update(lobotomy_counts: [u16; 12]) {
    let mut base_str = "lobotomy update! here are your stats:".to_string();
    for k in 0..12 {
        let count = lobotomy_counts[k];
        if count == 1 {
            base_str.push_str(&format!("\n1 {} ", KEYSTRINGS[k]));
        } else if count == 0 {
            base_str.push_str(&format!("\nNo {}", plural(KEYSTRINGS[k])));
        } else {
            base_str.push_str(&format!("\n{count} {}", plural(KEYSTRINGS[k])));
        }
    }
    lobotomy_messagebox(&base_str, "Fire in the hole!");
}


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // each millisecond theres a 1 in 10,000 chance of a lobotomy event
    // TODO: display dialog box "the lobotomy was successful! now you will start see some colourful things :)" with the fire in the hole icon

    let mut prev_keystates: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // previous keystates
    let mut lobotomy_counts: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // used statistics (todo: occasionally a window pops up with these stats displayed with the message "lobotomy update!")
    // println!("Hello, world!"); // line is here to make sure the program runs

    let mut lobotomy: bool = false;

    loop {
        let start = std::time::Instant::now();
        if rand::thread_rng().gen_range(0..10_000) == 1 {
            if !lobotomy {
                unsafe {init_lobotomy();}
                lobotomy = true;
            } else {
                unsafe {lobotomy_update(lobotomy_counts);}
            }

        }

        // runs every millisecond
        let mut current_keystates: [u16; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        unsafe {
            for key in 0..12 {
                current_keystates[key] = winapi::um::winuser::GetAsyncKeyState(KEYSTATES[key] as c_int) as u16;
            }
        }
        if lobotomy {
            let new_lobotomy_counts = virus(current_keystates, prev_keystates, &stream_handle);
            for k in 0..12 {
                lobotomy_counts[k] += new_lobotomy_counts[k];
            }
        }
        prev_keystates = current_keystates;
        let end = std::time::Instant::now();
        let elapsed = (end - start).as_micros();
        if elapsed < 1000 {
            std::thread::sleep(std::time::Duration::from_micros(1000 - elapsed as u64));
        }
    }
}

// yes im a geometry dash player lol