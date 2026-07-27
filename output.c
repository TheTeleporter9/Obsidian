#include <stdio.h>
#include <stdbool.h>

#include <stdlib.h>
#include <string.h>
int main() {
    int x = 10;
    int x2 = (x + 10);
    x = 11;
    float y = 3.6;
    printf("%s\n","Before expression");
    printf("%d\n",x);
    x = ((x + 2) - (((y * x2) / 10) * 100));
    printf("%s\n","After expression");
    printf("%d\n",x);
    char* coolName = "Hello";
    bool coolBool = false;
    coolBool = ((x >= (y - x2)) && (strcmp(coolName, "Hi") == 0));
    printf("%s\n", ((coolBool) ? "true" : "false"));
    printf("%s\n", (((strcmp(coolName, "Hello") == 0)) ? "true" : "false"));
    return 0;
}
