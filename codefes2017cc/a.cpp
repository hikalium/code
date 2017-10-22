#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  std::string s;
  std::cin >> s;
  int p = 0, q = s.length() - 1;
  int lastp = p, lastq = q;
  while (p <= q) {
    while (s[p] == 'x') p++;
    while (s[q] == 'x') q--;
    if (p > q) break;
    if (s[p] != s[q]) {
      std::cout << -1 << std::endl;
      return 0;
    }
    lastp = p;
    lastq = q;
    p++;
    q--;
  }
  uint64_t count = 0;
  p = lastp;
  q = lastq;
  while (1) {
    lastp--;
    lastq++;
    if (lastp < 0 && lastq >= s.length()) break;
    int pcnt = 0, qcnt = 0;
    while (lastp >= 0 && s[lastp] == 'x') {
      lastp--;
      pcnt++;
    }
    while (lastq < s.length() && s[lastq] == 'x') {
      lastq++;
      qcnt++;
    }
    count += abs(pcnt - qcnt);
  }
  std::cout << count << std::endl;

  return 0;
}
