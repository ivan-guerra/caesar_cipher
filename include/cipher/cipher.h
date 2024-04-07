#ifndef CIPHER_H_
#define CIPHER_H_

#include <iostream>

namespace cipher {

static constexpr int kAsciiAlphabetSize = 128; /**< Number of ASCII chars. */

/**
 * \brief Apply a Caesar Cipher to an ASCII text stream.
 * \details A Caesar Cipher with a shift of \p shift is applied to the ASCII
 *          input stream \p is. The result of the cipher is output character by
 *          character to the output stream \p os.
 * \param [in,out] is ASCII input stream.
 * \param [in,out] os ASCII output stream containing ciphered text.
 * \param [in] shift The Caesar Cipher shift (i.e., key) value.
 */
void AsciiCaesarCipher(std::istream &is, std::ostream &os, int shift);

}  // namespace cipher

#endif
