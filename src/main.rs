#![allow(non_snake_case)]
// Above is necessary due to json exportted keys from searchsploit (see to-do section)

/// grabsploit - the lazy pentester's searchsploit
/// author: brian d (@mox_folder_)
/// license: MIT OSL
/// free to modify w/ attribution preferred

/// Imports
use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// Constants
const TITLE_CUTOFF: usize = 64;

/// Structs

/// Struct to handle CLI arguments gracefully with `clap`
#[derive(Parser)]
struct CliArg {
    /// The exploit to look for
    exploit: String,
}

/// Searchsploit JSON result enum to store searchsploit's json output
/// Fields commented out are intentional, as I don't use them, feel free to add them back in
#[derive(Serialize, Deserialize, Debug)]
struct SearchsploitJson {
    //SEARCH: String,
    //DB_PATH_EXPLOIT: String,
    RESULTS_EXPLOIT: Vec<SearchsploitResult>,
    //DB_PATH_SHELLCODE: String,
    RESULTS_SHELLCODE: Vec<SearchsploitResult>,
}

/// Sub-enum to store the individual exploit results
/// Fields commented out are intentional, as I don't use them, feel free to add them back in
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchsploitResult {
    Title: String,
    EDB_ID: String,
    //Date: String,
    //Author: String,
    r#Type: String,
    Platform: String,
    Path: String,
}

/// Functions

/// Main program logic
fn main() {
    // Parse arguments & print welcome banner
    let args = CliArg::parse();
    print_banner();

    // Get output from searchsploit & build local cache
    let json_raw = run_searchsploit(&args.exploit);
    let search_results = parse_searchsploit_results(&json_raw);
    let cache = build_sploit_cache(search_results);

    // Display the cached results in sorted oreder
    show_results(cache.clone());

    // Get user input and grab the desired exploit to the local dir
    let key_selector: u32 = get_key_selector();
    grab_sploit(key_selector, cache);
}

// TODO
/// Print welcome banner dispayed when program is ran
fn print_banner() {
    let banner = "grabsploit v0.0 by @mox_folder_";
    println!("{}", banner);
}

/// Pass args to searchsploit and capture the result as a json dump string
fn run_searchsploit(exploit: &String) -> String {
    println!("Running `searchsploit {}`\n", exploit);

    // Configure the command and run it
    let command_child = Command::new("searchsploit")
        .args([exploit, "-j"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start the searchsploit process");

    // Get the output
    let command_output = command_child
        .wait_with_output()
        .expect("Failed to wait on searchsploit");
    let command_output = command_output.stdout;
    let command_output = std::str::from_utf8(command_output.as_slice())
        .unwrap()
        .to_string();
    let command_output = command_output.replace("EDB-ID", "EDB_ID"); // Required to access json field
    return command_output;
}

/// Use serde_json to deserialize the searchsploit json data string and return it
fn parse_searchsploit_results(json_data: &String) -> SearchsploitJson {
    return serde_json::from_str(json_data).unwrap();
}

// Redundant, but I like keys
/// Assign integer key values to exploits in hashmap "cache" for easy reference by user
fn build_sploit_cache(results: SearchsploitJson) -> HashMap<u32, SearchsploitResult> {
    let mut iter: u32 = 0;
    let mut sploit_cache = HashMap::new();

    for result in results.RESULTS_EXPLOIT {
        sploit_cache.insert(iter, result);
        iter += 1;
    }

    return sploit_cache;
}

/// Print all found results in formatted manner
fn show_results(cache: HashMap<u32, SearchsploitResult>) {
    for (key, result) in cache.iter().sorted_by_key(|x| x.0) {
        let mut title = result.Title.clone();
        if title.len() > TITLE_CUTOFF {
            title = title[0..TITLE_CUTOFF].to_string()
        };

        let key_format = format!(
            "{}{}{}",
            format!("[").blue(),
            format!("{}", key).green(),
            format!("]").blue()
        );
        let type_format = format!(
            "{}{}{}",
            format!("[").red(),
            format!("{}", result.Type).cyan(),
            format!("]").red()
        );
        println!("{}:{}:{}", key_format, type_format, title);
    }
}

/// Prompt for and store user input as an integer used to access data in the cache
fn get_key_selector() -> u32 {
    let mut user_input = String::new();
    print!("{}", format!("\nEnter sploit # to grab: ").magenta());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading from STDIN");
    return user_input.trim().parse::<u32>().unwrap() as u32;
}

/// Copy the desired exploit file to the current working directory
fn grab_sploit(key_selector: u32, cache: HashMap<u32, SearchsploitResult>) {
    println!(
        "\nGrabbing {} @ {}",
        cache[&key_selector].Title, cache[&key_selector].Path
    );

    let file_path = &cache[&key_selector].Path;
    let destination_file = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    match std::fs::copy(file_path, destination_file) {
        Err(e) => println!("Unexpected error when copying exploit: {:?}", e),
        _ => (),
    }

    println!(
        "Exploit successfully copied to {} in current directory...",
        destination_file
    );
}

//TODO
// fn read_sploit(sploit)

// TODO: implement input parsing to allow for quitting (e.g. entering 'q' instead of a number)
// TODO: implement terminal style input (arrow keys in particular)
// TODO: implement auto width cutoff for TITLE_CUTTOFF (move from const to static)
// TODO: highlight search term in results (like searchsploit does)
// TODO: graceful/kind error handling
// TODO: used built-in functionality of serde json to rename keys read from searchsploit to avoid using caps json keys
