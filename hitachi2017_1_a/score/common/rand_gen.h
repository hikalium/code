#include <random>

#ifndef RANDGEN
#define RANDGEN
struct Rand {  
public:
  Rand() = default;
  Rand(std::mt19937::result_type seed_input) : mt(seed_input) {}
  int NextInt(int a, int b) {
    return std::uniform_int_distribution<int>{a, b}(mt);
  }
private:
  std::mt19937 mt{std::random_device{}()};
};
#endif
