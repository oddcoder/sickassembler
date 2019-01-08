# Rusty Sick Assembler

[![Build Status](https://travis-ci.org/oddcoder/sickassembler.svg?branch=master)](https://travis-ci.org/oddcoder/sickassembler)

SIC/XE machine assembler

Extensive amount of error checking is done
- Undefined Mnemonics
- Undefined Directives
- Duplicate labels
- Control section redefinition
- Label redefinition
- Duplicate imports in a control section
- Duplicate exports of a variable
- Un-imported symbols
- Malformed instructions
- Invalid literals
- Out of bound addresses
- Invalid expressions
- Missing START instruction
- Missing Program name
- Illegal format addressing for imported symbol
- Out of bit range parameters

## Usage
The object file is generated in the same directory

```shell
Usage: target/debug/sick_assembler FILE [options] file

Options:
    -o, --output name   set output file name
    -c, --csect         print control section details
    -h, --help          print this help menu
```
## Tests
you can find test codes (.asm) files in `./src/tests/` directory

[![asciicast](https://asciinema.org/a/ixqOAryrJIV9meksHwpR7T30F.svg)](https://asciinema.org/a/ixqOAryrJIV9meksHwpR7T30F)
