#include <getopt.h>

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "cracker/cracker.h"

static void PrintUsage() {
  std::cout << "usage: ccracker [OPTION]..." << std::endl;
  std::cout << "find the key(s) with the highest probability of deciphering "
               "the ciphertext"
            << std::endl;
  std::cout << "\t-c, --ciphertext FILE\n\t\tfile containing ciphertext"
            << std::endl;
  std::cout << "\t-d, --dict-attack DICT_FILE\n\t\tperform a dictionary attack"
            << std::endl;
  std::cout << "\t-f, --freq-attack\n\t\tperform a frequency analysis attack"
            << std::endl;
  std::cout << "\t-h, --help\n\t\tprint this help page" << std::endl;
}

static void PrintErrorAndExit(const std::string &msg) {
  std::cerr << "error: " << msg << std::endl;
  std::exit(EXIT_FAILURE);
}

[[nodiscard]] static std::vector<int> FindKeys(
    const cracker::KeyScoreMap &scores) {
  int max_score = 0;
  std::vector<int> keys;
  for (const auto &[shift, score] : scores) {
    if (score > max_score) {
      keys.clear();
      max_score = score;
      keys.push_back(shift);
    } else if (score == max_score) {
      keys.push_back(shift);
    }
  }
  return keys;
}

static void PrintProbableKeys(const cracker::KeyScoreMap &scores) {
  std::vector<int> keys = FindKeys(scores);
  if (keys.empty()) {
    std::cout << "no viable key found" << std::endl;
  } else {
    std::cout << "most probable key(s): ";
    for (const int &key : keys) {
      std::cout << key << ' ';
    }
    std::cout << std::endl;
  }
}

static void DoDictAttack(std::istream &is, const std::string &dict_file) {
  std::ifstream dict_is(dict_file);
  if (!dict_is.good()) {
    PrintErrorAndExit(
        "unable to open dictionary file, verify you gave a valid path");
  }
  PrintProbableKeys(cracker::AsciiDictionaryAttack(is, dict_is));
}

static void DoFreqAttack(std::istream &is) {
  PrintProbableKeys(cracker::AsciiFrequencyAnalysisAttack(is));
}

int main(int argc, char **argv) {
  std::vector<struct option> longopts{
      {"ciphertext", required_argument, 0, 'c'},
      {"dict-attack", required_argument, 0, 'd'},
      {"freq-attack", no_argument, 0, 'f'},
      {"help", no_argument, 0, 'h'},
      {0, 0, 0, 0},
  };

  int opt = 0;
  int long_index = 0;
  std::string ciphertext, dict_file;
  bool do_freq_attack = false;
  while (-1 != (opt = ::getopt_long(argc, argv, "c:d:fh", &longopts[0],
                                    &long_index))) {
    switch (opt) {
      case 'c':
        ciphertext = optarg;
        break;
      case 'd':
        dict_file = optarg;
        break;
      case 'f':
        do_freq_attack = true;
        break;
      case 'h':
        PrintUsage();
        std::exit(EXIT_SUCCESS);
      case '?':
        std::exit(EXIT_FAILURE);
    }
  }

  if (!dict_file.empty() && do_freq_attack) {
    PrintErrorAndExit("you can only specify one attack algorithm per run");
  }

  std::ifstream cipher_is(ciphertext);
  std::istream &input_is = (ciphertext.empty()) ? std::cin : cipher_is;
  if (!input_is.good()) {
    PrintErrorAndExit("unable to open ciphertext file \"" + ciphertext + "\"");
  }

  /* Only perform a dictionary attack if explicitly told to do so. Otherwise,
   * perform a frequency analysis attack. */
  if (!dict_file.empty()) {
    DoDictAttack(cipher_is, dict_file);
  } else {
    DoFreqAttack(cipher_is);
  }

  std::exit(EXIT_SUCCESS);
}
