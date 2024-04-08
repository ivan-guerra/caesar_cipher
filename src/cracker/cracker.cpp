#include "cracker/cracker.h"

#include <array>
#include <cctype>
#include <cmath>
#include <limits>
#include <string>
#include <unordered_set>

#include "cipher/cipher.h"

namespace cracker {

using WordSet = std::unordered_set<std::string>;
using CharFrequencies = std::array<double, cipher::kAsciiAlphabetSize>;
using CharFrequencyArray =
    std::array<CharFrequencies, cipher::kAsciiAlphabetSize>;

[[nodiscard]] static WordSet LoadDictionary(std::istream& dict_is) {
  WordSet dictionary;
  std::string line;
  while (std::getline(dict_is, line)) {
    dictionary.insert(line);
  }
  return dictionary;
}

[[nodiscard]] static double ManhattanDist(const CharFrequencies& observed) {
  /* Source Material: tinyurl.com/4b6hdzh8 */
  static const CharFrequencies kAsciiFreqDistro = {
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      6.338218895840436e-08,
      0.0,
      0.0,
      0.0,
      1.2676437791680872e-07,
      0.019578060965172565,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      0.0,
      6.338218895840436e-08,
      0.0,
      0.0,
      6.338218895840436e-08,
      0.0,
      0.167564443682168,
      5.070575116672349e-07,
      0.0015754276887500987,
      0.0,
      5.070575116672349e-07,
      0.0,
      2.0282300466689395e-06,
      0.0015078622753204398,
      0.0003307916441739124,
      0.0003314254660634964,
      4.436753227088305e-07,
      1.5211725350017046e-06,
      0.008634492219614468,
      0.002076717421222119,
      0.011055184780313847,
      0.000519607185080999,
      0.005918945715880591,
      0.004937789430804492,
      0.002756237869045172,
      0.0021865587546870337,
      0.0018385271551164353,
      0.0025269211093936652,
      0.0019199098857390264,
      0.0018243295447897528,
      0.002552781042488694,
      0.002442242504945237,
      0.00012036277683200988,
      7.41571610813331e-06,
      0.00044107665296153596,
      2.5352875583361743e-07,
      0.0004404428310719519,
      4.626899793963519e-06,
      6.338218895840436e-08,
      0.0024774830020061096,
      0.0017387002075069484,
      0.002987392712176473,
      0.0010927723198318497,
      0.0012938206232079082,
      0.001220297284016159,
      0.0009310209736100016,
      0.0008752446473266058,
      0.0020910417959267183,
      0.0008814561018445294,
      0.0003808001912620934,
      0.0010044809306127922,
      0.0018134911904778657,
      0.0012758834637326799,
      0.0008210528757671701,
      0.00138908405321239,
      0.00010001709417636208,
      0.0011037374385216535,
      0.0030896915651553373,
      0.0030701064687671904,
      0.0010426370083657518,
      0.0002556203680692448,
      0.0008048270353938186,
      6.572732994986532e-05,
      0.00025194420110965734,
      8.619977698342993e-05,
      6.97204078542448e-07,
      0.0,
      6.338218895840436e-07,
      2.2183766135441526e-06,
      1.2676437791680872e-07,
      0.0,
      0.0612553996079051,
      0.01034644514338097,
      0.02500268898936656,
      0.03188948073064199,
      0.08610229517681191,
      0.015750347191785568,
      0.012804659959943725,
      0.02619237267611581,
      0.05480626188138746,
      0.000617596049210692,
      0.004945712204424292,
      0.03218192615049607,
      0.018140172626462205,
      0.05503703643138501,
      0.0541904405334676,
      0.017362092874808832,
      0.00100853739070613,
      0.051525029341199825,
      0.0518864979648296,
      0.0632964962389326,
      0.019247776378510318,
      0.007819143740853554,
      0.009565830104169261,
      0.0023064144740073764,
      0.010893686962847832,
      0.0005762708620098124,
      6.338218895840436e-08,
      0.0,
      0.0,
      1.9014656687521307e-07,
      3.1057272589618137e-06,
  };

  double sum = 0.0;
  for (int i = 0; i < cipher::kAsciiAlphabetSize; ++i) {
    sum += std::abs(observed[i] - kAsciiFreqDistro[i]);
  }
  return sum;
}

[[nodiscard]] static KeyScoreMap FindMinDistShifts(
    const CharFrequencyArray& freqs) {
  KeyScoreMap scores;
  double dist = 0.0;
  double min_dist = std::numeric_limits<double>::max();
  for (int shift = 0; shift < cipher::kAsciiAlphabetSize; ++shift) {
    dist = ManhattanDist(freqs[shift]);
    if (dist < min_dist) {
      scores.clear();
      scores[shift]++;
      min_dist = dist;
    } else if (dist == min_dist) {
      scores[shift]++;
    }
  }
  return scores;
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

KeyScoreMap AsciiFrequencyAnalysisAttack(std::istream& is) {
  CharFrequencyArray freqs;
  char curr = '\0';
  int tmp = 0;
  double num_chars = 0;
  while (is.get(curr)) {
    for (int shift = 0; shift < cipher::kAsciiAlphabetSize; ++shift) {
      /* Perform the Caesar Cipher shift. */
      tmp = (static_cast<int>(curr) + shift) % cipher::kAsciiAlphabetSize;

      /* Tally the shifted char. */
      freqs[shift][tmp]++;
    }
    num_chars++;
  }

  /* Calculate the percent frequency of each char. */
  for (auto& table : freqs) {
    for (double& val : table) {
      val /= num_chars;
    }
  }

  return FindMinDistShifts(freqs);
}

}  // namespace cracker
