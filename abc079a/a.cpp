#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int cost[10];
int dist[10][10];
int expanded[10];

void djkInit() {
  for (int i = 0; i < 10; i++) {
    cost[i] = 0xfffffff;
  }
  cost[1] = 0;
}

void djkExpand(int n) {
  for (int i = 0; i < 10; i++) {
    int tmp = cost[n] + dist[i][n];
    if (cost[i] > tmp) {
      cost[i] = tmp;
    }
  }
  expanded[n] = 1;
}

void djk() {
  djkExpand(1);
  for (;;) {
    int min = 0xfffffff, minIndex;
    for (int i = 0; i < 10; i++) {
      if (expanded[i]) continue;
      if (min > cost[i]) {
        min = cost[i];
        minIndex = i;
      }
    }
    if (min == 0xfffffff) {
      break;
    }
    djkExpand(minIndex);
  }
}

int main(int argc, char *argv[]) {
  int H, W;
  std::cin >> H >> W;
  int A[H][W];
  for (int y = 0; y < 10; y++) {
    for (int x = 0; x < 10; x++) {
      std::cin >> dist[y][x];
    }
  }
  for (int y = 0; y < H; y++) {
    for (int x = 0; x < W; x++) {
      std::cin >> A[y][x];
    }
  }
  djkInit();
  djk();
  int sum = 0;
  for (int y = 0; y < H; y++) {
    for (int x = 0; x < W; x++) {
      if (A[y][x] != -1) {
        sum += cost[A[y][x]];
      }
    }
  }
  std::cout << sum << std::endl;

  return 0;
}
