# impulser

impulse response generator

## Usage

``` sh
$ cargo run --release --example stdout 2> /dev/null > audio.raw
$ sox -t raw -e signed-integer -b 16 -r 44100 -c 1 audio.raw audio.wav
```

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2023 carrotflakes (carrotflakes@gmail.com)
