## Tofu - memory efficient modal terminal hex editor

(based on fork: https://github.com/lineality/teehee_noload_fork)

'Tofu' text editor (hex editor) loads from a file only up to 3x the size of what the terminal can show at a time, and uses a chunk-window system to move the window as the user scrolls.


## linux: for small build, use (for me executible is 1.8mb)
```bash
cargo build --profile release-small 
```

## ~Install
Set an executable file as a keyword in the command line interface (CLI) so that entering that keyword calls the executable:

1. Open the bash shell configuration file in a text editor. The configuration file is usually located at ~/.bashrc or ~/.bash_profile. (use whatever edictor: vim, nano, hx (helix), teehee, lapce, etc.)
```bash
hx ~/.bashrc
```
or in some systems it may be called 'b'ash_profile'

2. Add an alias for your executable at the end of the file. Replace your_executable with the name of your executable and /path/to/your_executable with the full path to your executable. Add:
```text
alias your_keyword='/path/to/your_executable'
```
e.g. add:
```text
alias tofu='/home/COMPUTERNAME/tofu_soft/tofu'
```

3. Save and close the text editor. 
- If you used nano, you can do this by pressing: Ctrl x s (control key, x key, s key)
- If you use Helix(hx), Vim(vi), or Teehee: 'i' to type, then esc for normal mode, then :wq to write and quit

4. Reload the bash shell configuration file to apply the changes.
```bash
source ~/.bashrc
```
or bash_profile


## Option debug_log
Turn a file's constant DEBUG_FLAG is set to boolean true, 
then a logs will be printed to a tofu_debug.text file in 
the current working directory.


# Original Teehee here:
https://github.com/Gskartwii/teehee/releases
#### For more information see the real Teehee (it's awsome)
Appologies if I did not correctly put attributes to the author in the .toml file;
Anything broken in this experimental fork is my fault and not the original author.

## Fork: This is an experimental fork of the original Teehee Gskartwii project.

## Implemented keybinds (from original version Readme)
* `hjkl` for movement (press shift to extend selection instead)
```
    ^
    k
< h   l >
    j
    v
```
* `g`[`hjkl`] for jumping (`G`[`hjkl`] to extend selection instead)
    * `h`: to line start
    * `l`: to line end
    * `k`: to file start
    * `j`: to file end
    * `<count>g` jumps to offset, `<count>G` extends to offset
* `<C+e/y>` to scroll down/up
* `;` to collapse selections to cursors
* `<a-;>` (alt and ;) to swap cursor and selection end
* `<a-s>` (alt and s) to split selection to multiple selections of size...
    * `b`: 1 byte
    * `w`: 2 bytes (Word)
    * `d`: 4 bytes (Dword)
    * `q`: 8 bytes (Qword)
    * `o`: 16 bytes (Oword)
    * `n`: delimited by null bytes
    * `/`: matching a text pattern (`?` for hex pattern)
* `d` to delete selected data from current_buffer
* `i` to enter insert mode at the beginning of selections (`I` to insert hex instead of ascii)
    * `a` instead of `i` to enter append mode instead
    * `o` instead of `i` to enter overwrite mode instead
    * `c` instead of `i` to delete selection contents, then enter insert mode
    * `<c-n>` to insert a null byte in ascii mode
    * `<c-o>` to switch between ascii and hex inserting
* `(` and `)` to cycle main selection
* `<space>` to keep only main selection, `<a-space>` to keep all selections but main
* `r<key>` to replace a each selected character with the ASCII character given
    * `R<digit><digit>` instead of `r` to replace with a single hex character instead
    * `r<c-n>` to replace with null bytes
* `y` to yank/copy selections to register `"`
* `p` to paste register `"` contents from `y`/`d`/`c`
* `s` to collapse selections to those matching a text pattern (`S` for hex pattern)
* `M` to measure length of current main selection (in bytes)
* `u` to undo, `U` to redo
* `:` to enter command mode
    * `:q` to quit
    * `:q!` to force quit (even if current_buffer dirty)
    * `:w` to flush current_buffer to disk
    * `:w <filename>` to save current_buffer to named file
    * `:wa` to flush all buffr_collection to disk
    * `:e <filename>` to open a new current_buffer
    * `:db` to close a current_buffer
    * `:db!` to close a current_buffer even if dirty
    * `:wq` to flush current_buffer, then quit

Entering a pattern:

* `<C-w>` to insert a wildcard
* `<C-o>` to switch input mode (ascii <-> hex)
* `<esc>` to go back to normal mode
* `<enter>` to accept pattern
* arrow keys, `<backspace>` and `<delete>` also supported

Counts:
* The following commands maybe prefixed by a count:
    * Movement (`hjkl` and `HJKL`)
    * Selection modification (`()<space><a-space>`)
    * Jump to offset (`g` and `G`)
    * Paste (`p`)
    * (In split mode) `bwdqon`
* Counts are inputted by typing digits 0-9 (in hex mode, 0-f).
* `x` switches between hex and decimal mode.
* Note that `a-f` may shadow some keys, so switch out of hex mode before running
  a command.
* Example: `y5p`: yank the selection and paste it 5 times.
* Example: `50l`: Move 50 bytes to the right.
* Example: `x500g`: Jump to offset 0x500
* Example: `<a-s>x12xb`: Split selection into parts of 0x12 bytes.


# Fork Goals:
1. No-Load Memory: [maybe Done]
Based on an astute issue posted on the original Teehee git repo:
```
xeruf commented on May 21, 2021
"...One of my main use-cases for hex editors is fixing issues with partitions.
Partitions can be huge, ..."
```
The primary goal of Tofu (and the No-Load Teehee Fork) is 
having the editor only load for view what is needed to view.

Data Science is another area where it is very common to have multi-gigabyte 
files that are too big to open with most software...but you still need to inspect
the file.

2. (will try in future) Being able to open to a "place" in the file, which may not have 'lines' by %

