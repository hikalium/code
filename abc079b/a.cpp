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
  std::string s;
  std::cin >> s;
  std::cout << a + b + c << " " << s << std::endl;

  return 0;
}
