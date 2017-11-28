#include <cstdio>
#include <vector>
#include <set>
#include <algorithm>
#include <utility>
#include <cassert>
#include <string>
#include "../common/constraints.h"

struct Element {
  int from, to, cost;
  Element(int a, int b, int c) : from(a), to(b), cost(c) {}
};

int main(int argc, char** argv) {
  if(argc != 3) {
    fprintf(stderr, "Usage: %s <in-file> <out-file>\n", argv[0]);
    return 1;
  }
  
  // read files
  FILE *in, *out;
  if((in = fopen(argv[1], "r")) == NULL) {
    fprintf(stderr, "Error (cannot open): %s\n", argv[1]);
    return 1;
  }
  if((out = fopen(argv[2], "r")) == NULL) {
    fprintf(stderr, "Error (cannot open): %s\n", argv[2]);
    return 1;
  }

  std::vector<Element> edges_orig;
  std::set< std::pair<int, int> > find_multi, edges_emb;
  int sum = 0, score = 0, temp;

  try {
    // read G
    int V, E;
    fscanf(in, "%d%d", &V, &E);
    for(int i=0; i<E; i++) {
      int u, v, w;
      if(fscanf(in, "%d%d%d", &u, &v, &w) == EOF)    throw 0;
      if(u >= v)                                     throw 5;
      u--; v--;
      if(u < 0 || u >= V || v < 0 || v >= V)         throw 3;
      if(w < MIN_C || w > MAX_C)                     throw 3;
      std::pair<int, int> edge(u, v);
      if(find_multi.count(edge))                     throw 6;
      find_multi.insert(edge);
      edges_orig.push_back(Element(u, v, w));
      sum += w;
    }

    // read G_emb
    int V_emb, E_emb;
    fscanf(in, "%d%d", &V_emb, &E_emb);

    for(int i=0; i<E_emb; i++) {
      int a, b;
      if(fscanf(in, "%d%d", &a, &b) == EOF)          throw 0;
      if(a >= b)                                     throw 5;
      a--; b--;
      if(a < 0 || a >= V_emb || b < 0 || b >= V_emb) throw 3;
      std::pair<int, int> edge(a, b);
      if(edges_emb.count(edge))                      throw 6;
      edges_emb.insert(edge);
    }
    if(fscanf(in, "%d", &temp) != EOF)               throw 0;

    // Constraints
    // 1. The number of lines on output must be equal to |V|
    // 2. \phi is injective function, so if i \neq j, a_i \neq a_j, and b_i \neq b_j
    std::vector<int> phi(V, -1), used(V_emb);

    // read output-file
    for(int i=0; i<V; i++) {
      int s, t;
      if(fscanf(out, "%d%d", &s, &t) == EOF)         throw 1;
      s--; t--;
      if(s < 0 || s >= V || t < 0 || t >= V_emb)     throw 4;
      if(phi[s] >= 0 || used[t])                     throw 2;
      phi[s] = t;
      used[t] = 1;
    }
    if(fscanf(out, "%d", &temp) != EOF)              throw 1;

    // calculate
    for(int i=0; i<E; i++) {
      Element e = edges_orig[i];
      int x = phi[e.from], y = phi[e.to];
      if(x > y) std::swap(x, y);
      std::pair<int, int> edge = std::make_pair(x, y);
      if(edges_emb.count(edge)) score += e.cost;
    }
  }

  // exception
  catch (int mode) {
    // 0, 1
    if(mode < 2) {
      std::string s = (mode ? "output" : "input");
      fprintf(stderr, "Error: the number of lines is invalid (%s)\n", s.c_str());
    }
    // 2
    else if(mode == 2) {
      fprintf(stderr, "Error: \"phi\" must be injective function\n");
    }
    // 3, 4
    else if(mode < 5) {
      mode -= 3;
      std::string s = (mode ? "output" : "input");
      fprintf(stderr, "Error: out of range (%s)\n", s.c_str());
    }
    // 5
    else if(mode == 5) {
      fprintf(stderr, "Error: for the edge (u, v), v must be strictly greater than u (input)\n");
    }
    // 6
    else if(mode == 6) {
      fprintf(stderr, "Error: given graph must not have multiple edge (input)\n");
    }
  }

  printf("[Score]/[Sum]: %7d/%7d\n", score, sum);
  return 0;
}
