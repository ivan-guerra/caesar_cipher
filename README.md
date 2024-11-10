# Caesar Cipher Tools

This repo contains [Caesar Cipher][1] encode, decode, and cracking utilities.
These utilities only support the ASCII character set.

### Encoding/Decoding Messages

The `ccipher` utility encodes/decodes messages. Below is the `ccipher` usage
message:

```text
Caesar Cipher encryption/decryption utility.

Usage: ccipher [OPTIONS] <KEY>

Arguments:
  <KEY>  encryption/decryption key

Options:
  -i, --input-file <INPUT_FILE>    input plaintext/ciphertext file
  -o, --output-file <OUTPUT_FILE>  output plaintext/ciphertext file
  -h, --help                       Print help
  -V, --version                    Print version
```

`ccipher` accepts input from `STDIN` or from a file if one is specified via the
`--input-file` option. Ciphertext is output to `STDOUT` by default but can also
be written to file via shell redirection or the `--output-file` option.

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
outputs the key that most probably decrypts the text. You can take the
ciphertext and the key output by `ccracker` and use them as input to `ccipher`
to retrieve the secret message.

Below is the `ccracker` usage message:

```
Caesar cipher cracker.

Usage: ccracker [OPTIONS]

Options:
  -i, --ciphertext-file <CIPHERTEXT_FILE>
          file containing ciphertext
  -a, --attack <ATTACK>
          attack type [default: dictionary] [possible values: dictionary, frequency]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print versiontext
```

`ccracker` reads ciphertext input from `STDIN` by default. Optionally, you can
supply the ciphertext in a file using the `--ciphertext-file` option. `ccracker`
implements two cracking algorithms: a dictionary attack and a frequency analysis
attack. The dictionary attack is the default option. You can chose to run a
frequency attack with the `--attack frequency` option.

Below is an example of cracking a message using the dictionary attack:

```text
echo "&#**-H" | ./ccracker
```

The output of the above command is `candidate key: 66`. We can apply this key to
the ciphertext using `ccipher`:

```text
echo "&#**-H" | ./ccipher -k 66
```

The output will be the plaintext message `hello`!

### References

- [Popular English Words Dictionary][2]
- [ASCII Character Frequency Table][3]

[1]: https://en.wikipedia.org/wiki/Caesar_cipher#
[2]: https://github.com/dolph/dictionary
[3]: https://github.com/piersy/ascii-char-frequency-english?tab=readme-ov-file
