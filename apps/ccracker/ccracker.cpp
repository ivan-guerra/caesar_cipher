#include <getopt.h>

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "cracker/cracker.h"

static void PrintUsage() noexcept {
  std::cout << "usage: ccracker [OPTION]... DICT_PATH" << std::endl;
  std::cout << "find the key(s) with the highest probability of deciphering "
               "the ciphertext"
            << std::endl;
  std::cout << "\t-c, --ciphertext FILE\n\t\tfile containing ciphertext"
            << std::endl;
  std::cout << "\t-h, --help\n\t\tprint this help page" << std::endl;
}

static void PrintErrorAndExit(const std::string &msg) noexcept {
  std::cerr << "error: " << msg << std::endl;
  std::exit(EXIT_FAILURE);
}

[[nodiscard]] static std::vector<int> FindKeys(
    const cracker::KeyScoreMap &scores) noexcept {
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

static void PrintProbableKeys(const cracker::KeyScoreMap &scores) noexcept {
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

int main(int argc, char **argv) {
  std::vector<struct option> longopts{
      {"ciphertext", required_argument, 0, 'c'},
      {"help", no_argument, 0, 'h'},
      {0, 0, 0, 0},
  };

  int opt = 0;
  int long_index = 0;
  std::string ciphertext;
  while (-1 !=
         (opt = ::getopt_long(argc, argv, "c:h", &longopts[0], &long_index))) {
    switch (opt) {
      case 'c':
        ciphertext = optarg;
        break;
      case 'h':
        PrintUsage();
        std::exit(EXIT_SUCCESS);
      case '?':
        std::exit(EXIT_FAILURE);
    }
  }

  if (!argv[optind]) {
    PrintErrorAndExit("missing dictionary file path");
  }
  std::ifstream dict_is(argv[optind]);
  if (!dict_is.good()) {
    PrintErrorAndExit(
        "unable to open dictionary file, verify you gave a valid path");
  }

  std::ifstream cipher_is(ciphertext);
  std::istream &input_is = (ciphertext.empty()) ? std::cin : cipher_is;
  if (!input_is.good()) {
    PrintErrorAndExit("unable to open ciphertext file \"" + ciphertext + "\"");
  }

  cracker::KeyScoreMap scores =
      cracker::AsciiDictionaryAttack(input_is, dict_is);

  PrintProbableKeys(scores);

  std::exit(EXIT_SUCCESS);
}
