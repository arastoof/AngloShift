# AngloShift
This is a Rust based tool that is used to easily convert spellings between their British and American variants.

## Usage
In the command line, to directly use the tool, you can do something like
``` bash
cargo run -- -t 'The color of My tyre is grey.' -b
```
to convert all of the British spellings in the text to their British variants.

use `-a` instead of `-b` to convert to American.

To load in a file, you can do something like
``` bash
cargo run -- -t example.txt -b
```