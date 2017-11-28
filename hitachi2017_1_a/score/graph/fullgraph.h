#include <cstdio>
#include "../common/rand_gen.h"

void gen_fullgraph(FILE *fp, int V, int min_cost, int max_cost, int seed) {
  Rand rnd_full(seed);
  int E = V * (V - 1) / 2;
  fprintf(fp, "%d %d\n", V, E);
  
  for(int i=0; i<V; i++) {
    for(int j=i+1; j<V; j++) {
      int s = i+1, t = j+1;
      int cost = rnd_full.NextInt(min_cost, max_cost);
      fprintf(fp, "%d %d %d\n", s, t, cost);
    }
  }
}
