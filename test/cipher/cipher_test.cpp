#include "cipher/cipher.h"

#include <gtest/gtest.h>

#include <sstream>

static const int kKey = 127;

static const char* kPlaintext = "hello\n";
static const char* kCiphertext = "gdkkn\t";

TEST(CipherTest, AsciiCaesarCipherWithEmptyInputReturnsSuccess) {
  std::istringstream is;
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess, cipher::AsciiCaesarCipher(is, os, kKey));
  ASSERT_TRUE(os.str().empty());
}

TEST(CipherTest, AsciiCaesarCipherWithBadInputReturnsBadInputStream) {
  std::istringstream is;
  is.setstate(std::ios_base::badbit);
  ASSERT_FALSE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kBadInputStream,
            cipher::AsciiCaesarCipher(is, os, kKey));
}

TEST(CipherTest, AsciiCaesarCipherWithBadOutputReturnsBadOutputStream) {
  std::istringstream is(kPlaintext);
  ASSERT_TRUE(is);

  std::ostringstream os;
  os.setstate(std::ios_base::failbit);
  ASSERT_FALSE(os);
  ASSERT_EQ(cipher::RetCode::kBadOutputStream,
            cipher::AsciiCaesarCipher(is, os, kKey));
}

TEST(CipherTest, AsciiCaesarCipherEncrypt) {
  std::istringstream is(kPlaintext);
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess, cipher::AsciiCaesarCipher(is, os, kKey));
  ASSERT_EQ(os.str(), kCiphertext);
}

TEST(CipherTest, AsciiCaesarCipherDecrypt) {
  std::istringstream is(kCiphertext);
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess,
            cipher::AsciiCaesarCipher(is, os, -kKey));
  ASSERT_EQ(os.str(), kPlaintext);
}
