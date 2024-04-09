#include "cracker/cracker.h"

#include <gtest/gtest.h>

#include <fstream>
#include <string>

static const int kExpectedKey = 27;

static const char* kDictionaryFile = "../dictionaries/popular.txt";
static const char* kEmptyFile = "data/empty.txt";
static const char* kSingleWordCipherFile = "data/single_word_ciphertext.txt";
static const char* kMultiwordCipherFile = "data/multiword_ciphertext.txt";
static const char* kMultilineCipherFile = "data/multiline_ciphertext.txt";

static void TestFrequencyAnalysisAttack(const std::string& ciphertext_file,
                                        int expected_key) {
  std::ifstream is(ciphertext_file);
  ASSERT_TRUE(is);

  cracker::KeyScoreMap scores = cracker::AsciiFrequencyAnalysisAttack(is);
  ASSERT_TRUE(scores.count(expected_key));
}

static void TestDictionaryAttack(const std::string& ciphertext_file,
                                 int expected_key) {
  std::ifstream is(ciphertext_file);
  ASSERT_TRUE(is);

  std::ifstream dict_is(kDictionaryFile);
  ASSERT_TRUE(dict_is);

  cracker::KeyScoreMap scores = cracker::AsciiDictionaryAttack(is, dict_is);
  ASSERT_TRUE(scores.count(expected_key));
}

TEST(CrackerTest, AsciiFreqAttackReturnsEmptyKeyMapOnBadStream) {
  std::ifstream bad_is;
  bad_is.setstate(std::ios_base::badbit);
  ASSERT_FALSE(bad_is);

  cracker::KeyScoreMap scores = cracker::AsciiFrequencyAnalysisAttack(bad_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiFreqAttackReturnsEmptyKeyMapOnEmptyStream) {
  std::ifstream empty_is(kEmptyFile);
  ASSERT_TRUE(empty_is);

  cracker::KeyScoreMap scores = cracker::AsciiFrequencyAnalysisAttack(empty_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiDictAttackReturnsEmptyKeyMapOnBadStream) {
  std::ifstream bad_is;
  bad_is.setstate(std::ios_base::badbit);
  ASSERT_FALSE(bad_is);

  std::ifstream dict_is(kDictionaryFile);
  ASSERT_TRUE(dict_is);

  cracker::KeyScoreMap scores = cracker::AsciiDictionaryAttack(bad_is, dict_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiDictAttackReturnsEmptyKeyMapOnEmptyStream) {
  std::ifstream empty_is(kEmptyFile);
  ASSERT_TRUE(empty_is);

  std::ifstream dict_is(kDictionaryFile);
  ASSERT_TRUE(dict_is);

  cracker::KeyScoreMap scores =
      cracker::AsciiDictionaryAttack(empty_is, dict_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiDictAttackReturnsEmptyKeyMapOnBadDictStream) {
  std::ifstream is(kSingleWordCipherFile);
  ASSERT_TRUE(is);

  std::ifstream dict_is("nonexistent_file");
  ASSERT_FALSE(dict_is);

  cracker::KeyScoreMap scores = cracker::AsciiDictionaryAttack(is, dict_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiDictAttackReturnsEmptyKeyMapOnEmptyDictStream) {
  std::ifstream is(kSingleWordCipherFile);
  ASSERT_TRUE(is);

  std::ifstream dict_is(kEmptyFile);
  ASSERT_TRUE(dict_is);

  cracker::KeyScoreMap scores = cracker::AsciiDictionaryAttack(is, dict_is);
  ASSERT_TRUE(scores.empty());
}

TEST(CrackerTest, AsciiFreqAttackFindsKeyForSingleWordInput) {
  TestFrequencyAnalysisAttack(kSingleWordCipherFile, kExpectedKey);
}

TEST(CrackerTest, AsciiFreqAttackFindsKeyForMultiwordInput) {
  TestFrequencyAnalysisAttack(kMultiwordCipherFile, kExpectedKey);
}

TEST(CrackerTest, AsciiFreqAttackFindsKeyForMultilineInput) {
  TestFrequencyAnalysisAttack(kMultilineCipherFile, kExpectedKey);
}

TEST(CrackerTest, AsciiDictAttackFindsKeyForSingleWordInput) {
  TestDictionaryAttack(kSingleWordCipherFile, kExpectedKey);
}

TEST(CrackerTest, AsciiDictAttackFindsKeyForMultiwordInput) {
  TestDictionaryAttack(kMultiwordCipherFile, kExpectedKey);
}

TEST(CrackerTest, AsciiDictAttackFindsKeyForMultilineInput) {
  TestDictionaryAttack(kMultilineCipherFile, kExpectedKey);
}
