#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

std::vector<int> nums;

int main(int argc, char *argv[]) {
  int N;

  std::cin >> N;

  for (int i = 0; i < N; i++) {
    int v;
    std::cin >> v;
    nums.push_back(v);
  }

  int maxDiff = -1000000000;
  int kmax = nums[N - 1] - 1;
  for (int k = N - 1; k >= 0; k--) {
    if (nums[k] <= kmax) continue;
    kmax = nums[k];
    for (int p = k - 1; p >= 0; p--) {
      if (nums[k] - nums[p] > maxDiff) {
        maxDiff = nums[k] - nums[p];
      }
    }
  }
  std::cout << maxDiff << std::endl;

  return 0;
}
