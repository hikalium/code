#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int dpsolve(std::string &s) {
  int hash[s.length()];
  int mindp[s.length()];

  for (int i = 0; i < s.length(); i++) {
    hash[i] = (1 << (s[i] - 'a'));
    if (i) hash[i] ^= hash[i - 1];
    mindp[i] = s.length();
    // std::cout << hash[i] << std::endl;
  }
  mindp[0] = 1;
  for (int x = 1; x < s.length(); x++) {
    for (int i = x - 1; i >= 0; i--) {
      int bits = hash[x] ^ hash[i];
      if ((bits & (bits - 1)) == 0) {
        mindp[x] = std::min(mindp[x], mindp[i] + 1);
      }
    }
  }
  /*
  for (int x = 1; x < s.length(); x++) {
    std::cout << mindp[x] << std::endl;
  }
  */
  return mindp[s.length() - 1];
}

int main(int argc, char *argv[]) {
  srand(time(NULL));
  std::string s;
  std::cin >> s;
  /*
    for (int i = 0; i < 3000; i++) {
      s += 'a' + rand() % ('z' - 'a');
    }
    */
  std::cout << dpsolve(s) << std::endl;
  return 0;
}
