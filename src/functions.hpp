//
// Created by d.evsyukov on 09.01.2020.
//

#ifndef T_FUNCTIONS_HPP
#define T_FUNCTIONS_HPP

#include "sha256/picosha2.h"
#include <string>

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

std::vector<std::string> split(const std::string &s, char delimiter) {
  std::vector<std::string> tokens;
  std::string token;
  std::istringstream tokenStream(s);
  while (std::getline(tokenStream, token, delimiter)) {
    tokens.push_back(token);
  }
  return tokens;
}

#endif // T_FUNCTIONS_HPP
