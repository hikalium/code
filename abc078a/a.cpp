#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  int X, Y, Z;
  std::cin >> X >> Y >> Z;
  X -= Z;
  if (X < 0) X = 0;
  std::cout << (X / (Y + Z)) << std::endl;

  return 0;
}
