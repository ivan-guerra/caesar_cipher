#include "cipher/cipher.h"

#include <gtest/gtest.h>

#include <fstream>
#include <sstream>

static const int kKey = 101;

static const char* kSingleWordPlaintextFile = "data/single_word_plaintext.txt";
static const char* kSingleWordCipherFile = "data/single_word_ciphertext.txt";

static const char* kMultiwordPlaintextFile = "data/multiword_plaintext.txt";
static const char* kMultiwordCipherFile = "data/multiword_ciphertext.txt";

static const char* kMultilinePlaintextFile = "data/multiline_plaintext.txt";
static const char* kMultilineCipherFile = "data/multiline_ciphertext.txt";

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
  std::istringstream is(kSingleWordPlaintextFile);
  ASSERT_TRUE(is);

  std::ostringstream os;
  os.setstate(std::ios_base::failbit);
  ASSERT_FALSE(os);
  ASSERT_EQ(cipher::RetCode::kBadOutputStream,
            cipher::AsciiCaesarCipher(is, os, kKey));
}

TEST(CipherTest, AsciiCaesarCipherSingleWordInput) {
  std::ifstream is(kSingleWordPlaintextFile);
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess, cipher::AsciiCaesarCipher(is, os, kKey));

  std::string cipher;
  std::ifstream cipher_is(kSingleWordCipherFile);
  std::getline(cipher_is, cipher);
  ASSERT_EQ(os.str(), cipher);
}

TEST(CipherTest, AsciiCaesarCipherMultiwordInput) {
  std::ifstream is(kMultiwordPlaintextFile);
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess, cipher::AsciiCaesarCipher(is, os, kKey));

  std::string cipher;
  std::ifstream cipher_is(kMultiwordCipherFile);
  std::getline(cipher_is, cipher);
  ASSERT_EQ(os.str(), cipher);
}

TEST(CipherTest, AsciiCaesarCipherMultilineInput) {
  std::ifstream is(kMultilinePlaintextFile);
  ASSERT_TRUE(is);

  std::ostringstream os;
  ASSERT_EQ(cipher::RetCode::kSuccess, cipher::AsciiCaesarCipher(is, os, kKey));

  std::string cipher;
  std::ifstream cipher_is(kMultilineCipherFile);
  std::getline(cipher_is, cipher);
  ASSERT_EQ(os.str(), cipher);
}
