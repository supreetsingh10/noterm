#  NOTERM STICKY NOTES FOR GNU/LINUX 

Sticky notes for your terminal, written in rust. 


## Dependencies
1. Cargo

## Building the project & Running the project
1. Go to the project directory. 
2. Run the following command 
> cargo build --release
3. Go to `target/release` directory and the required binary would be called `noterm`.
4. The binary `noterm` can be added to the `/usr/share/binary`, adding it to the PATH. 

## User documentation 
When the command is first run, it checks for a config.json file at `~/.config/noterm/`, if the file does not exist it
creates that file. If the command is run with `pin` subcommand first, it takes the values and adds them to the
`~/.config/noterm`

Note index is a value that is displayed on the top of your note, in the following commands you will refer to the notes
using the note index only. 

### noterm show 
Displays the saved notes in the config file, with the predefined values. 

### noterm pin -m / --msg [message text] -c / --col [Color of the note]
No need to fix the position and coordinates for the specific notes, the algorithm takes care of it. The values will be
written in the config file so they will persist as long as you don't delete them. 

### noterm update -n / --note [note index] -m / msg [message text] -c / --col [color of the note]
If the note index value is not given it will through an error, and safely close the program.

You can update the message text and color of the note, with this command.

### noterm delete -n [note index]
Deletes the specific note from the config. 

### Colors available for notes. 
Ratatui library was used for making the UI for the application. The list of colours available can be seen from their
documentation. [colors](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html)
