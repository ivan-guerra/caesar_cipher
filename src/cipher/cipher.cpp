#include "cipher/cipher.h"

#include <iostream>

namespace cipher {

RetCode AsciiCaesarCipher(std::istream &is, std::ostream &os, int shift) {
  if (!is) {
    return RetCode::kBadInputStream;
  }
  if (!os) {
    return RetCode::kBadOutputStream;
  }

  /* Negative shifts get adjusted to be positive. The ciphertext winds up being
   * the same as if we had directly applied the negative shift. */
  if (shift < 0) {
    shift += kAsciiAlphabetSize;
  }

  char curr = '\0';
  while (is.get(curr)) {
    curr = (static_cast<int>(curr) + shift) % kAsciiAlphabetSize;
    os << curr;
  }
  os.flush(); /* Flush the output stream just to be safe. */

  return RetCode::kSuccess;
}

}  // namespace cipher
