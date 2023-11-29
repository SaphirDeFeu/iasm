# iasm : The Assembly Interpreter
*Side note: Not actual assembly  
### Project by SaphirDeFeu
(they couldn't find an assembler easy enough so they wrote their "own")  
# How can I download the Assembly interpreter?
Currently, there isn't any way to use iasm on Linux or MacOS.  

I am planning on adding support for these systems in the future if I can manage to get my hands on a way to do that.
### > Windows Method 1
- Download `iasm.exe`
- Add the parent folder of `iasm.exe` to PATH
  - Open the Windows Search menu
  - Search for "Edit the system environment variables"
  - Click on "Environment variables" (bottom of dialog box)
  - In the "System variables" category, search for "Path"
  - "Edit" > "New"
  - Write the path to the folder where you downloaded iasm
  - "Ok" until you don't have anymore dialog boxes.
- Once you have done that, you can use the `iasm` command straight out the box!  

### > Windows Method 2
- Download `iasm.exe`
- Place the executable file into the directory you wish to use it
- Run `./iasm.exe` when you want to run the interpreter

# How does it work?
IASM supports multiple commands that closely mimic the look and feel of assembly, without the fear of destroying your software that easily.
## *WARNING: IASM is <ins>NOT</ins> designed for actual software development.*
To run iasm, you have to specify a file path for your file in the command (`iasm path/to/file.s`)  
IASM supports multiple CLI options that you can use to personnalize the way that iasm runs.  
### Creating an IASM program
An IASM program always starts at the first `main` label the interpreter runs into.  
For example, here is what the simplest iasm program would look like :
```
main:
  ret 0
```
Starts the program and exits with code 0.  
Each command is designed to be very basic.  
Here is an in-depth tutorial on how each command works :  
## `ret <code>`
Exits the program with the specified return value `<code>`
## `lda <value>`
Loads a value into the accumulator.  
Said value may be presented under different forms:  
- `#<value>` indicates an immediate value: load this value into the accumulator
- `<value>` indicates a stored value: load the value at address <value> into the accumulator
- `:<value>` indicates a pointer value: load the value that is referenced by the value at address <value> into the accumulator
  - Example case: `lda :0` will load the value at address 0 (let's say it's 2), and with that loaded value (here, 2), will load the value located at address 2 (because in this case, the value at address 8 is 2, so load address 2).  

Additional information is that values can take up the form of hexadecimal or binary format using the syntax `$<hex value>` or `%<bin value>`.  
## `sta <address>`
Stores the accumulator's value at address `<address>`
## `pha`
Pushes the accumulator's value onto the stack
## `pla`
Pulls the current stack value and loads it into the accumulator
## `add <value>`
Adds a specified value to the accumulator's value.  
Values follow the syntax of `lda`.
## `sub <value>`
Subtracts a specified value to the accumulator's value.
Values follow the syntax of `lda`.
## `cmp <address1> <address2>`
Compares the values at `<address1>` and `<address2>`. If they're equal, then the `zero` flag is set to true. Otherwise is set to false.  
If the value at `<address1>` is less than the value at `<address2>`, then the `less` flag is set to true. Otherwise, is set to false.  
If the value at `<address1>` is less than the value at `<address2>`, then the `more` flag is set to true. Otherwise, is set to false.
## `jmp <label|token>`
Jumps to a specified label or token (It is recommended to use labels).
## `beq <label|token>`
Used in conjunction to `cmp`. Does the same thing as `jmp` but only when the `zero` flag is true (if `cmp` was successful).
## `bne <label|token>`
Inverse of `beq`.
## `blt <label|token>`
Jumps if the value 1 is less than the value 2 after `cmp`
## `bgt <label|token>`
Jumps if the value 1 is greater than the value 2 after `cmp`
## `jsr <label|token>`
Does the same thing as `jmp` but can be used to return from the jump (see `rsr`).
## `rsr`
Returns to the token where `jsr` left off. Basically a `return` statement from a function.
## `and <value>`
Takes the bits of the value in the accumulator and `and`'s them with the specified `<value>`
## `not`
If the value in the accumulator is 1, it will be set to 0.  
If it's 0, it will be set to 1.  
Otherwise, it'll throw an error. To invert multiple bits, use the `xor` operation (see below).
## `xor <value>`
Takes the bits of the value in the accumulator and `xor`'s them with the specified `<value>`
## `or <value>`
Takes the bits of the value in the accumulator and `or`'s them with the specified `<value>`
## `shl <value>`
Shifts the bits in the accumulator by a specified amount `<value>` to the left
## `shr <value>`
Shifts the bits in the accumulator by a specified amount `<value>` to the right
## `rol <value>`
Rotates the bits in the accumulator by a specified amount `<value>` to the left (take into account the fact that the accumulator uses 32-bit)
## `ror <value>`
Rotates the bits in the accumulator by a specified amount `<value>` to the right (take into account the fact that the accumulator uses 32-bit)
## `nop [time]`
Stops execution for a specified amount of time. If no time is specified, then execution is stopped for the minimal amount of time.
## `mov <reg> <value>`
`mov` registers are specific to iasm (See table below for details). Used to move a value into the register `reg` (or out of it for `cin`).
- `cout <values...>`
  - Outputs the corresponding ascii character from values. Supports multiple types of output formatting :
  - `cout "some text here"` will output text.
  - `cout 72 89 65` will output the corresponding unicode characters of each number (72=H)
  - `cout :8` will output the corresponding unicode character of the value contained at the address specified after the colon (in this case, address 8).
  - These formats can be used in conjunction with one another.
- `cin <start address>`
  - Takes in an input from the user and stores each character into an address number starting from the start address.
  - If the user inputs "ME 2" and the start address is set to 0, then the program will write "M" at address 0, "E" at address 1, " " at address 2, "2" at address 3 and will place a null byte at address 4.