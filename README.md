## OBS Subtitle server
Solution for streaming subtitles in OBS using a browser source updating from a local web server, that gets updated subtitle lines from a python script.

Subtitle text files in `py_scripts/subs/*.txt` , with subtitle lines in plaintext

Local web server uses ports 3000 and 8080

### Rust implementation
Someone wrote a superfast™ Rust implementation of the subtitle script. Run it like this:
First, put subtitles in `py_scripts/subs/*.txt`, then:
- [Install Rust](https://www.rust-lang.org/tools/install)
- Maybe install `pkg-config` and `libssl-dev` apt packages if using Debian/Ubuntu
- `cd txt2post`
- `cargo run`

Keyboard shortcuts:
- Space: Show next line
- b: Show previous line
- c: Clear screen
- v: Disable/enable sending requests
- m: Go to menu
- q: Quit

How to write subtitles:
- Use comments (#) to show lines in terminal but not on the website
- Use [ ] enclosed comments to show blank lines
- Use / as as newline
- Use :: to separate bottom/left/top/right parts
- Copy text from Docs to subs/raw.txt and run `ǹpm run convert` to create subs/undertekster.txt
- Open subtitles with `npm run start`