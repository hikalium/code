#include <cstdlib>
#include <cstdio>
#include <string>
#include "../graph/kingsgraph_square.h"
#include "../graph/fullgraph.h"
#include "../graph/randomgraph.h"
#include "../common/rand_gen.h"
#include "../common/constraints.h"

int read_string(char *str) {
  char* end_point;
  int ret_number = strtol(str, &end_point, 10);
  if(*end_point != '\0') {
    fprintf(stderr, "Error: invalid input: \"%s\"\n", str);
    exit(1);
  }
  return ret_number;
}

int main(int argc, char** argv) {
  if(argc != 4) {
    fprintf(stderr, "Usage: %s <filename> <graphclass> <seed>\n", argv[0]);
    fprintf(stderr, "( <graphclass> \\in {0: random graph, 1: full graph} )\n");
    return 1;
  }

  FILE *fp = fopen(argv[1], "w");
  if(fp == NULL) {
    fprintf(stderr, "Error: cannot open the file\n");
    return 1;
  }

  int V = 0, E = 0, N = 0;
  int case_type = read_string(argv[2]);
  int seed      = read_string(argv[3]);
  Rand rnd(seed);

  if     (case_type == 0) {
    // random graph
    V = rnd.NextInt(MIN_V, MAX_V);
    E = rnd.NextInt(V-1, std::min(V*(V-1)/2, MAX_E));
    gen_randomgraph(fp, V, E, MIN_C, MAX_C, seed);
  }
  else if(case_type == 1) {
    // full graph
    V = rnd.NextInt(MIN_V_full, MAX_V_full);
    gen_fullgraph(fp, V, MIN_C, MAX_C, seed);
  }
  else {
    // type error
    fprintf(stderr, "Error: case-type number is invalid (you must choose 0 or 1)\n");
    return 1;
  }

  while(1) {
    N = rnd.NextInt(MIN_N, MAX_N);
    // ensure |V_emb| >= |V|
    if(N*N >= V) break;
  }
  gen_kingsgraph(fp, N);
  fclose(fp);
  return 0;
}
