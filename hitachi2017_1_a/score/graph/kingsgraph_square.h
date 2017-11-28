#include <cstdio>

void gen_kingsgraph(FILE *fp, int N) {
  // output N, and the number of vertices and edges
  int V = N * N;
  int E = 4*N*N - 6*N + 2;
  fprintf(fp, "%d %d\n", V, E);

  // output King's graph

  //  _
  // |x <- create this shape ( (N-1)*(N-1) times )

  int dx1[] = {0, 0, 0, 0};
  int dy1[] = {0, 0, 0, 1};

  int dx2[] = {0, 1, 1, 1};
  int dy2[] = {1, 0, 1, 0};

  for(int i=0; i<N-1; i++) {
    for(int j=0; j<N-1; j++) {
      int current_v = i*N + (j+1);
      for(int k=0; k<4; k++) {
        int start_v = current_v + dx1[k]*N + dy1[k];
        int goal_v  = current_v + dx2[k]*N + dy2[k];
        if(start_v > goal_v) std::swap(start_v, goal_v);
        fprintf(fp, "%d %d\n", start_v, goal_v);
      }
    }
  }

  // create | ( (N-1) times)
  for(int i=0; i<N-1; i++) {
    int start_v = (i+1) * N;
    int goal_v  = (i+2) * N;
    fprintf(fp, "%d %d\n", start_v, goal_v);
  }

  // create - ( (N-1) times)
  for(int i=0; i<N-1; i++) {
    int start_v = (N-1)*N + (i+1);
    int goal_v  = start_v + 1;
    fprintf(fp, "%d %d\n", start_v, goal_v);
  }
}
