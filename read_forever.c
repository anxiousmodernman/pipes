#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>

int main() {

    char buf[256];
    int n;
     
    while (1) {
        
        n = read(STDIN_FILENO, buf, sizeof(buf));
        if (n < 0) {
            printf("error from read\n");
            exit(1);
        }
        if (n == 0) {
            printf("got EOF\n");
            exit(0);
        }
        
        write(STDOUT_FILENO, "read: ", 6); // we know it's 6 bytes
        write(STDOUT_FILENO, buf, n);

    }
}