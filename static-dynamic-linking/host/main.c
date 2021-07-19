#include "../lib/lib.h"
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, const char **argv) {

  if (argc != 2) {
    printf("Usage: %s path to dylib\n", argv[0]);
    exit(1);
  }

  char *error;

  void *lib = dlopen(argv[1], RTLD_LAZY | RTLD_LOCAL);

  if ((error = dlerror())) {
    printf("Error: %s\n", error);
    exit(1);
  }

  void (*plugin)(void) = dlsym(lib, "");

  if ((error = dlerror())) {
    printf("Error: %s\n", error);
    exit(1);
  }
  
  execute("Task 1");
  plugin();


  dlclose(lib);
}
