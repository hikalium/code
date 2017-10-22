#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  int H, W;
  std::cin >> H >> W;
  char inMap[H][W];
  char calcMap[H][W];
  for (int y = 0; y < H; y++) {
    for (int x = 0; x < W; x++) {
      std::cin >> inMap[y][x];
      calcMap[y][x] = 0;
    }
  }
  for (int y = 0; y < H; y++) {
    for (int x = 0; x < W; x++) {
      if (inMap[y][x] != '#') continue;
      if (y > 0) calcMap[y - 1][x]++;
      if (x > 0) calcMap[y][x - 1]++;
      if (x < W - 1) calcMap[y][x + 1]++;
      if (y < H - 1) calcMap[y + 1][x]++;
      if (y > 0 && x > 0) calcMap[y - 1][x - 1]++;
      if (y > 0 && x < W - 1) calcMap[y - 1][x + 1]++;
      if (y < H - 1 && x > 0) calcMap[y + 1][x - 1]++;
      if (y < H - 1 && x < W - 1) calcMap[y + 1][x + 1]++;
    }
  }
  for (int y = 0; y < H; y++) {
    for (int x = 0; x < W; x++) {
      if (inMap[y][x] == '#')
        std::cout << "#";
      else
        std::cout << (int)calcMap[y][x];
    }
    std::cout << std::endl;
  }
  return 0;
}
