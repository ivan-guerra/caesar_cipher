#include "cracker/cracker.h"

#include <cctype>
#include <string>
#include <unordered_set>

#include "cipher/cipher.h"

namespace cracker {

using WordSet = std::unordered_set<std::string>;

[[nodiscard]] WordSet LoadDictionary(std::istream& dict_is) {
  WordSet dictionary;
  std::string line;
  while (std::getline(dict_is, line)) {
    dictionary.insert(line);
  }
  return dictionary;
}

KeyScoreMap AsciiDictionaryAttack(std::istream& is, std::istream& dict_is) {
  WordSet dictionary = LoadDictionary(dict_is);
  KeyScoreMap scores;
  char tmp = '\0';
  std::string words[cipher::kAsciiAlphabetSize];
  std::string line;
  while (std::getline(is, line)) {
    for (const char& curr : line) {
      for (int shift = 0; shift < cipher::kAsciiAlphabetSize; ++shift) {
        /* Perform the Caesar Cipher shift. */
        tmp = (static_cast<int>(curr) + shift) % cipher::kAsciiAlphabetSize;

        if (std::isalnum(tmp)) { /* Add a char to the word at this shift. */
          words[shift] += std::tolower(tmp);
        } else if (!words[shift].empty() &&
                   std::isspace(tmp)) { /* Found complete word. */
          if (dictionary.count(words[shift])) {
            scores[shift]++;
          }
          words[shift].clear();
        }
      }
    }
  }

  /* Check for the trailing words. */
  for (int shift = 0; shift < cipher::kAsciiAlphabetSize; ++shift) {
    if (dictionary.count(words[shift])) {
      scores[shift]++;
    }
  }
  return scores;
}

}  // namespace cracker
