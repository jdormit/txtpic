# txtpic
> Generate Unicode art from images

## Installation
You'll need [Cargo](http://doc.crates.io) to install `txtpic`.

    $ cargo install txtpic

## Usage

    txtpic 0.1.0
    Jeremy Dormitzer <jeremy.dormitzer@gmail.com>
    Generates text representations of images

    USAGE:
    txtpic [OPTIONS] <IMAGE>

    FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

    OPTIONS:
    -c, --character-set <CHARACTERS>    An alternate character set to use
    -w, --width <WIDTH>                 An approximate width value for the result

    ARGS:
    <IMAGE>    The input image

## Example
Here is an adorable cat.

![Adorable cat](./example/cat.jpg)

Converting it with:

    $ txtpic --width 100 cat.jpg

Gives the [output](./example/cat.txt):

![Text cat](./example/text-cat.png)
