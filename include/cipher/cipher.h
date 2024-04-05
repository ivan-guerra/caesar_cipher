#ifndef CIPHER_H_
#define CIPHER_H_

#include <iostream>

namespace cipher {

void AsciiCaesarCipher(std::istream &is, std::ostream &os, int shift) noexcept;

}  // namespace cipher

#endif
