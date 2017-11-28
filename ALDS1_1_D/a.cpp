#include <stdint.h>
#include <algorithm>
#include <bitset>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

typedef struct LKLIST LkList;
struct LKLIST {
  LkList *next;
  int v;
};

int main(int argc, char *argv[]) {
  LkList head;
  head.next = 0;
  int N;

  std::cin >> N;

  for (int i = 0; i < N; i++) {
    std::string cmd;
    std::cin >> cmd;
    if (cmd == "insert") {
      LkList *oldhead = head.next;
      head.next = new LkList();
      head.next->next = oldhead;
      std::cin >> head.next->v;
    } else if (cmd == "delete") {
      int target;
      std::cin >> target;
      for (LkList **e = &head.next; *e; e = &(*e)->next) {
        if ((*e)->v == target) {
          LkList *toDel = (*e);
          *e = toDel->next;
          // delete toDel;
        }
      }
    } else if (cmd == "deleteFirst") {
      LkList **e = &head.next;
      LkList *toDel = (*e);
      *e = toDel->next;
      delete toDel;
    } else if (cmd == "deleteLast") {
      for (LkList **e = &head.next; *e; e = &(*e)->next) {
        if (!(*e)->next) {
          LkList *toDel = (*e);
          *e = toDel->next;
          delete toDel;
        }
      }
    }
    for (LkList **e = &head.next; *e; e = &(*e)->next) {
      if (e != &head.next) {
        std::cout << " ";
      }
      std::cout << (*e)->v;
    }
    std::cout << std::endl;
  }

  return 0;
}
