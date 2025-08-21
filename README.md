# TeX Stripper
*James D Pickering, 2023. Distributed under GNU General Public Licence v3.0 (see below).*  

`tex_stripper` is a simple program that allows specific environments (figures, equations, blocks, etc) from within large TeX documents to be "stripped" from that document and placed into a new TeX file. 

The initial intended use case was to strip all figure, table, and block environments from a set of text-heavy lecture notes and automatically insert them into a beamer presentation. The resulting beamer presentation can then be used when delivering the lectures as a set of slides, requiring minimal extra effort once the notes are written. However, it is flexible enough to be usable for many other applications. Existing resources doing similar things tend to be more limited in their use cases.

`tex_stripper` is written in Rust by a Rust noob, and compiled binaries are provided.

## Usage 
`tex_stripper` can either be run using the pre-compiled binaries or by compiling and running from the Rust itself.

### Basic
Basic use of `tex_stripper` is to run the compiled binary from the command line: 
```
./tex_stripper -i FILE_TO_STRIP -o OUTPUT_FILE -f FILE --boxes-ignore ENVIRONMENTS

```
Of the command line arguments, the only required arguments are: 
- `-i FILE_TO_STRIP`  - the input .tex file that you want to strip environments from.
- `-o OUTPUT_FILE` - where `tex_stripper` should write the output (created if not present).

Optional arguments are:
- `-f FILE` - this allows you to specify a file containing lines of TeX that will be placed before and after each stripped environment in the final document, and at the start and end of the document. This file must conform to the example given (four blocks of TeX separated by comments starting with '%#' (hash to allow the TeX blocks to contain comments)). 
- `--boxes-ignore ENVIRONMENTS` - this allows you specify a list of environments to **not** strip from your input .text file. 

For example, you might want to create a beamer presentation `presentation.tex` containing environments from a large document `document.tex`, but excluding equations, itemizes, and enumerates. You can place the boilerplate TeX that comes at the start and end of your presentation file in an input file `strip_input.txt` and pass it using the -f flag. Similarly, you can add boilerplate TeX that surrounds each extracted figure environment with a beamer frame environment to the same file so that running `tex_stripper` produces a production-ready beamer presentation in one hit. You would then run:
```
tex_stripper -i document.tex -o presentation.tex -f strip_input.tex --boxes-ignore equation itemize enumerate
```

### Advanced 

The underlying rust code is reasonably thoroughly commented and is provided for further use under The GNU Public Licence V3. Compiling and running yourself via `cargo` is fairly straightforward. 
## Notes

- Currently `tex_stripper` can only strip environments that start and end with a `\begin{}` and `\end{}` statement. 
- Currently `tex_stripper` cannot natively unpack `\include{}` statements in an input file. Use another tool like the Perl script `latexpand` (https://www.ctan.org/pkg/latexpand) on a master file to unpack it before running `tex_stripper`.

## Bugs

Please report bugs on GITHUB LINK.

## Contact

Contact me at EMAIL

## Licence
Copyright 2023 J D Pickering

This file is part of `tex_stripper`.

`tex_stripper` is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

`tex_stripper` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with `tex_stripper`. If not, see <https://www.gnu.org/licenses/>.


## Contact

Contact me at EMAIL
