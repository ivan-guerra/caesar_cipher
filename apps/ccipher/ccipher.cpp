#include <getopt.h>

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "cipher/cipher.h"

static void PrintUsage() noexcept {
  std::cout << "usage: ccipher --key KEY [OPTION]..." << std::endl;
  std::cout << "encrypt/decrypt ASCII text via Caesar Cipher" << std::endl;
  std::cout << "\t-k, --key KEY\n\t\tcipher key (REQUIRED)" << std::endl;
  std::cout << "\t-i, --infile FILE\n\t\tinput file path" << std::endl;
  std::cout << "\t-o, --outfile FILE\n\t\toutput file path" << std::endl;
  std::cout << "\t-h, --help\n\t\tprint this help page" << std::endl;
}

static void PrintErrorAndExit(const std::string &msg) noexcept {
  std::cerr << "error: " << msg << std::endl;
  std::exit(EXIT_FAILURE);
}

int main(int argc, char **argv) {
  std::vector<struct option> longopts{
      {"key", required_argument, 0, 'k'},
      {"infile", required_argument, 0, 'i'},
      {"outfile", required_argument, 0, 'o'},
      {"help", no_argument, 0, 'h'},
      {0, 0, 0, 0},
  };

  int opt = 0;
  int long_index = 0;
  int key = 0;
  bool set_key = false;
  std::string infile, outfile;
  while (-1 != (opt = ::getopt_long(argc, argv, "k:i:o:h", &longopts[0],
                                    &long_index))) {
    switch (opt) {
      case 'k':
        set_key = true;
        key = std::stoi(optarg);
        break;
      case 'i':
        infile = optarg;
        break;
      case 'o':
        outfile = optarg;
        break;
      case 'h':
        PrintUsage();
        std::exit(EXIT_SUCCESS);
      case '?':
        std::exit(EXIT_FAILURE);
    }
  }

  if (!set_key) {
    PrintErrorAndExit("missing cipher key (include the '--key KEY' option)");
  }

  std::ifstream ifile_handle(infile);
  std::istream &is = (infile.empty()) ? std::cin : ifile_handle;
  if (!is.good()) {
    PrintErrorAndExit("unable to open infile \"" + infile + "\"");
  }

  std::ofstream ofile_handle(outfile);
  std::ostream &os = (outfile.empty()) ? std::cout : ofile_handle;
  if (!os.good()) {
    PrintErrorAndExit("unable to open outfile \"" + outfile + "\"");
  }

  cipher::AsciiCaesarCipher(is, os, key);

  std::exit(EXIT_SUCCESS);
}
