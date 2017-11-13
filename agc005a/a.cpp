#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

std::string s;
int check(int p, int q) { return 0; }

int main(int argc, char *argv[]) {
  std::cin >> s;

  std::cout << check(0, s.length()) << std::endl;

  return 0;
}
