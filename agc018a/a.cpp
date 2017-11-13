#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  int64_t Q, H, S, D;
  std::cin >> Q >> H >> S >> D;
  int cs, cd;
  int64_t N;
  std::cin >> N;
  cd = N >> 1;
  cs = N & 1;
  S = std::min(S, std::min(Q * 4, H * 2));
  D = std::min(2 * S, D);
  std::cout << cd * D + cs * S << std::endl;

  return 0;
}
