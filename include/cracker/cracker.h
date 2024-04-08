#ifndef CRACKER_H_
#define CRACKER_H_

#include <iostream>
#include <unordered_map>

namespace cracker {

using KeyScoreMap = std::unordered_map<int, int>;

/**
 * \brief Perform a dictionary attack on the Caesar Cipher stream \p is.
 * \param [in] is An ASCII input stream containing ciphertext generated
 *                by a Caesar Cipher.
 * \param [out] dict_is An input stream containing a newline separated list
 *                      of words.
 * \return A #KeyScoreMap whose keys represent Caesar Cipher keys and whose
 *         values represent the number of dictionary words detected when
 *         applying the key to the ciphertext stream \p is. The higher the
 *         score of a key, the more likely it can decrypt the ciphertext.
 */
[[nodiscard]] KeyScoreMap AsciiDictionaryAttack(std::istream& is,
                                                std::istream& dict_is);

/**
 * \brief Perform a frequency analysis attack on the Caesar Cipher stream \p is.
 * \param [in] is An ASCII input stream containing ciphertext generated
 *                by a Caesar Cipher.
 * \return A #KeyScoreMap whose keys represent Caesar Cipher keys. The map's
 *         values can only be 0 or 1. If 1, the character frequency
 *         distribution most closely matches the theoretical ASCII character
 *         frequency distribution indicating the assoicated shift value is
 *         the key.
 */
[[nodiscard]] KeyScoreMap AsciiFrequencyAnalysisAttack(std::istream& is);

}  // namespace cracker

#endif
