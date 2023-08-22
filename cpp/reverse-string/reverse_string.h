//
// Created by Takashi Idobe on 2019-07-09.
//
#include <string>
#include <iostream>
#include <algorithm>

#ifndef REVERSE_STRING__REVERSE_STRING_H_
#define REVERSE_STRING__REVERSE_STRING_H_

namespace reverse_string {
  std::string reverse_string(std::string s) {
    std::reverse(s.begin(), s.end());
    return s;
  }
}

#endif //REVERSE_STRING__REVERSE_STRING_H_
