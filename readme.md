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

Gives the output:

<pre style="color: white; background-color: #222">
==xxxxxxxxxx==xxx==x==+==---+++-=+-.:---:-:-+-.:-++----+==+==xxxxxxxxx==x===xxxxxxxx==============++
==xxxxxxxxx=====xx====++++++++--=+:.-+--:--++--:-+++++-++++++==xxxxxxx=++++=xxxxxxxx===============+
=xxxxxxxxxx=====xxxxx===+++++++++++:+=+-:--+x=+--++==++-+=+======+======+--==+=xxxxx====x===========
=xxxxxx==+===xxxxxxxx=+++++++++=+---+x=-:-+=x+--+=x++++++++====+++=++++----+--==xxx=====x===========
=======x=++=xxxxx==++++++=++++++==-:-xx-::+xx+:-=xx+-+++===========++-+++-:---+=xx==================
===++++=+---===+++++===++=+++--+xx=+-==---+xx+.-xxx=+---xxxxx====x=+++--++-::--+====================
++++++-++-:-+++++++========++-:-+xxx+-x+--+xx-.+xxxx+:-+=xx=====+===++++-+++----==============++++++
--++--++--:-++++++====x===xx=+:.+xxx-:+=-:-+x+:+xxxx+:-+x=++=========++++++=++=+==xx================
-+--+++-+---+++=++=x==+++++=x+-.:xxx+--=---+=+-+=xxx=--xx=xxx==xxxx=+++++++==+====xxxxxxx===========
+--++++++++++++====xx==+++++=x+..xxxx=-=+---++++-xxxx++x=++++==++=++++++++=+=++++==xxxxxxxxxxxxxxxxx
--+++-++===========xxxx====xxxx--xx==+-=+++-++==++=xxxx=++:+xx%x+-=+++++++--++=++=xxxxxxxxxxxxxxxxxx
+++++-++++=x====+==x==-+xx=--+=x==x=+-+++++++++x==xxxxxxx=+-xxxx+-=x=+==++=+--+===xxxxxxxxxxxxxxxxxx
+++-+-==++=x====++++==+xxxx--=xxxxxx=+=+=++++=+===x%=-xxxxxxxxx+-=x=+++++=xx=+-+==xxxxxxxxxxxxxxxxxx
+++---+=+++=++xxxxxxxx-=xxxxxxxxx+:xxx===+++=+====xx=.:-=xx==+--=x+----====xx=====xx%%%%%%%%%%%%%%%%
+++---++==++========xx=-+xxxxxxx+..=xx==+++++++====x=+------++++-----+===xxxxxxx=xxx%%%%%%%%%%%%%%%%
++++--+==---=xx===+++=xx=++++--:-++++=+===========+----+==+-::----++===xxxxxxx=+=xxxxxxxxxxxxxxxxxxx
++-----+====xx==+++++++--+++===xx=--::+xxxx===xx===+:..--::-----+++==xx=+---++-+xxxxxxxxxxxxxxxxxxxx
++++-++==xxxxxxxxxx==++-+--------+-..-+==xxx==xx==+=-::------+++++=xxxxxx====+=xxxx%%%%%%%%%%%%%%%%%
+-++--+++++xxxxxxxx======+++--+-+--::+++=========+-+++---+++++++=xx=xxxx=====xxxxxxxxxxxxxxxxxxxxx%%
++----++++++++++++=xxxxxx=++++++++--+---+=======++--+++-++==+==xxxxxxx====xxxx%%%%%%%%%%%%%%%%%%%xxx
+==++--+==++++===+=====xxx======++++=+---====+=++=+--+====+====xxxxxx===xxxxxxxxx%%%%%%%%%%%%%%%%%%%
+===++++++==++++==x==xxxxxxxxx=====xx=--+=++++++++++-=xxx=++=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
-+=++++++++====+++++===xxxxxxx=++=xxxx=+=x=======+=+xxxx+xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
---+++++++-++============xxxxxxx====xxxxx=+-=x==-=xxxxxx===xxxxxxxxxxxxxxxxx=xxxxxxxxxxxxxxxxxxxxxxx
+--++++----++=x=====xx===xxxxxxxx===xxxxxxx==+-+=xxxxxxxxxxxxxxxxxxxxxxx=====xxxx==xxxxxxxxxxxxxxxxx
+++--+++++=++-+=x=xxxxx=====xxxxxxxxxxxxxxxxx+-xxxxxxxxxxxxxxxxxxxxxxxx=++=+++++++=xxxxxxxxxxxxxxxxx
=++++---++++++++======+===xxxxxxxxxxxxxxxxxxx=-+xxxxxxxxxxxxxxxxx=xxxxx===++++---+======xxxxxxxxxxxx
===++----+==+++++++++==xxxx=xxxxxx=xxxxxxxxxxxxxxxxxxxx%xxxxxxxxxxxxxx===++++---++======+=xxxxxxxxxx
++++=+-+-----+++++++===+++=xxxxx==+=+=xxx%xxxxxxxxxxxxxxxxxxxxxxxxxxx==++++++---+==x======xxxxxxxxxx
+++-+++--:------++===+--+=++=+++-+-+---++==xx%%xx%%xxxxxxxxxxxxxxxxx==+++----++++==xxxxx=+xxxxxxxxxx
+++++=++--+-::--+++++-+==+==++++++++-+-+++===xxxxxx=======xxxxxxxxx=+++----::-+==+==xxxxx=xxxxxxxxxx
+++==++++++-+++--+----=+=++++++++=+++++++============++===+=xxxxxxx==++--::::-++===xxxxxxxxxxxxxxxxx
++==++-+---+++-------+++--+=++++++==++=++=++====x==x=xxxxxxxxxxxxxx=++--------++++x=xxxxxxxxxxxxxxxx
</pre>
