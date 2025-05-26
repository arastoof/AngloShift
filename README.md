# AngloShift
This is a Rust based tool that is used to easily convert spellings between their British and American variants.

## Usage
In the command line, to directly use the tool, you can do something like
``` bash
cargo run -- -t 'The color of My tyre is grey.' -b
```
to convert all of the American spellings in the text to their British variants.

use `-a` instead of `-b` to convert to American.

To load in a file, you can do something like
``` bash
cargo run -- -t example.txt -b
```

## Credit
The files `american_spellings.json` and `british_spellings.json` were taken from [American-British-English-Translator](https://github.com/hyperreality/American-British-English-Translator).