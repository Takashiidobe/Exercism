//
// Created by Takashi Idobe on 2019-07-09.
//

#ifndef LEAP__LEAP_H_
#define LEAP__LEAP_H_

namespace leap {
  bool is_leap_year(const int& year) {
    if (year % 4 == 0) {
      if (year % 400 == 0) {
        return true;
      } else if (year % 100 == 0) {
        return false;
      } else {
        return false;
      }
    } else {
      return false;
    }
  }
}

#endif //LEAP__LEAP_H_
