#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  int a, b, c;
  std::cin >> a >> b >> c;
  std::cout << (a ^ b ^ c) << std::endl;

  return 0;
}
