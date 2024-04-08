# Caesar Cipher Tools

This repo contains [Caesar Cipher][1] encode, decode, and cracking utilities.
These utilities only support the ASCII character set.

### Building

Included is a build script that supports a number of command line options:

```text
build caesar cipher tools
usage: build.sh [OPTION]...
options:
	-g    enable debug info
	-t    build unit tests
	-d    build doxygen docs
	-h    print this help message
```

You will need the following libraries and tools to build:

* CMake3.16+
* C++ compiler supporting C++20 features
* [Doxygen][2]

To build, run the `build.sh` script with arguments of your choosing. For
example, to build all utilities, Doxygen docs, and unit tests:

```bash
cd caesar_cipher/scripts
./build.sh -d -t
```

Build artefacts install to the following locations:

* Binaries: `caesar_cipher/bin`
* Doxygen Docs: `caesar_cipher/docs/html`
* Unit Tests: `caesar_cipher/build`

### Encoding/Decoding Messages

The `ccipher` utility encodes/decodes messages. Below is the `ccipher` usage
message:

```text
usage: ccipher --key KEY [OPTION]...
encrypt/decrypt ASCII text via Caesar Cipher
	-k, --key KEY
		cipher key (REQUIRED)
	-i, --infile FILE
		input file path
	-o, --outfile FILE
		output file path
	-h, --help
		print this help page
```

`ccipher` accepts input from `STDIN` or from a file if one is specified via the
`--infile` option. Ciphered text is output to `STDOUT` by default but can also
be written to file via shell redirection or the `--outfile` switch.

In the example below, we encrypt the contents of a file, `plaintext`, containing
the string `hello` using the key `27`. The output of the encrypt operation is
sent to the file `ciphertext`. Another call to `ccipher` with a key of `-27`
decrypts the contents of `ciphertext` to `STDOUT`.

```text
echo "hello" > plaintext
./ccipher -k 27 -i plaintext -o ciphertext
./ccipher -k -27 -i ciphertext
```

### Code Cracking 

The `ccracker` utility takes as input ciphertext produced by a Caesar Cipher and
outputs the key or keys that most probably decrypt the text. You can take the
ciphertext and the key(s) output by `ccracker` and use them as input to
`ccipher` to retrieve the secret message.

Below is the `ccracker` usage message:

```text
usage: ccracker [OPTION]...
find the key(s) with the highest probability of deciphering the ciphertext
	-c, --ciphertext FILE
		file containing ciphertext
	-d, --dict-attack DICT_FILE
		perform a dictionary attack
	-f, --freq-attack
		perform a frequency analysis attack
	-h, --help
		print this help page
```

`ccracker` reads ciphertext input from `STDIN` by default. Optionally, you can
supply the ciphertext in a file using the `--ciphertext` option. `ccracker`
implements two cracking algorithms: a dictionary attack and a frequency analysis
attack. The frequency analysis attack is the default option. You can chose to
run a dictionary attack with the `--dict-attack` option. A dictionary attack
requires that you pass `ccracker` a dictionary file containing newline seperated
words. A file containing the most popular English words is included in this repo
under `caesar_cipher/dictionaries/popular.txt`.

Below is an example of cracking a message using the dictionary attack:

```text
echo "&#**-H" | ./ccracker -d ../dictionaries/popular.txt
```

The output of the above command is `most probable key(s): 66`. We can apply this
key to the ciphertext using `ccipher`:

```text
echo "&#**-H" | ./ccipher -k 66
```

The output will be the plaintext message `hello`!

### Unit Testing

Included in this repo are number of GoogleTest unit tests for both the cipher
and cracker utilities. Follow the steps below to build and run the unit tests.

1. Change directory to `caesar_cipher/scripts/`.

2. Run the build script with the `-t` option:
```bash
./build.sh -t
```

3. Change directory to `caesar_cipher/build/`.

4. Run CTest:
```bash
ctest
```

### Doxygen Docs 

Project source is documented using Doxygen. To build the docs, follow these
steps:

1. Change directory to `caesar_cipher/scripts/`.

2. Run the build script with the `-d` option:
```bash
./build.sh -d
```

3. Open `caesar_cipher/docs/html/index.html` in your browser.

### References

* [Popular English Words Dictionary][3]
* [ASCII Character Frequency Table][4]

[1]: https://en.wikipedia.org/wiki/Caesar_cipher#
[2]: https://www.doxygen.nl/
[3]: https://github.com/dolph/dictionary 
[4]: https://github.com/piersy/ascii-char-frequency-english?tab=readme-ov-file
