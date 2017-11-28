#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  int N;

  std::cin >> N;

  int nums[N];

  for (int i = 0; i < N; i++) {
    std::cin >> nums[i];
  }

  int swapCount = 0;
  for (int i = 0; i < N - 1; i++) {
    int minIndex = i + 1;
    for (int k = i + 2; k < N; k++) {
      if (nums[minIndex] > nums[k]) {
        minIndex = k;
      }
    }
    if (nums[i] > nums[minIndex]) {
      int v = nums[minIndex];
      nums[minIndex] = nums[i];
      nums[i] = v;
      swapCount++;
    }
  }
  for (int i = 0; i < N; i++) {
    if (i) std::cout << " ";
    std::cout << nums[i];
  }
  std::cout << std::endl;
  std::cout << swapCount << std::endl;

  return 0;
}
