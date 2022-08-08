# grabsploit

## About
This is probably redundant, or it's probably been done before. But I wanted to practice rust so I made this.

`grabsploit` is meant to be a shorthand binary for `searchsploit`.

Typically, when using searchsploit, the workflow is this:
1. Run your searchsploit query
2. Look at the output and try to identify what will work best for your current goals
3. Run `less` (or whatever) to peak at the code (return to step 1 or 2 depending on if you're on the right path)
4. Either copy the code manually over to your working dir or use `searchsploit -m /some/long/path/here/that/you/have/to/highlight/with/your/mouse`

With grabsploit you can simply operate from your keyboard:

1. Run your grabsploit query
2. Enter the # of the exploit you want to use
3. Press enter

Voila! It's in your working directory.

An inline `less` (or less-like feature) is on the to-do. So, in the future you can peak at the code, decide whether to *grab the sploit* or go peak at other results, and so on. 

## Building
1. Follow instructions [here](https://www.rust-lang.org/tools/install) to install rust if you haven't already
2. From project directory, run `cargo build` 
3. Copy `$projectDir$/target/debug/grabsploit` to a secure location in your user's path to run it from anywhere, any time

## Usage
To use: simply run grabsploit with your desired searchsploit search term as its arg (just as you would searchsploit)
`grabsploit eternal`

For multiword searches, wrap your search term in quotes:
`grabsploit "eternal blue"

## To Do
[In no particular order]

- Update json structs and parsing to replace searchsploit json keys with forbidden chars and convert to snek case
- Add graceful error handling
- Highlight original search term in results as searchsploit does
- Implement auto terminal width detection on launch, move `TITLE_CUTOFF` to static, and set the cut off accordingly
- Implement handling for arrow keys / other standard terminal inputs to avoid raw bytes
- Add quit/exit option at exploit selection prompt
- Add reading functionality (inline `less` command for exploits) to allow users to quickly peak at code before they decide to grab it
- Add optional console loop that lets users look at exploits individually, go back to results, or even run another search (aka improve the UX)
- Add optional filtering by exploit type (e.g. `shellcode`, `webapps`, `remote`, etc)
