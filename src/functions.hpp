//
// Created by d.evsyukov on 09.01.2020.
//

#ifndef T_FUNCTIONS_HPP
#define T_FUNCTIONS_HPP

#include "sha256/picosha2.h"

std::string &ltrim(std::string &str, const std::string &chars = "\t\n\v\f\r ") {
  str.erase(0, str.find_first_not_of(chars));
  return str;
}

std::string &rtrim(std::string &str, const std::string &chars = "\t\n\v\f\r ") {
  str.erase(str.find_last_not_of(chars) + 1);
  return str;
}

std::string &trim(std::string &str, const std::string &chars = "\t\n\v\f\r ") {
  return ltrim(rtrim(str, chars), chars);
}

std::string sha256_hash(std::string text) {
  std::vector<unsigned char> hash(picosha2::k_digest_size);
  picosha2::hash256(text.begin(), text.end(), hash.begin(), hash.end());

  std::string hex_str = picosha2::bytes_to_hex_string(hash.begin(), hash.end());
  return hex_str;
}
#endif // T_FUNCTIONS_HPP
