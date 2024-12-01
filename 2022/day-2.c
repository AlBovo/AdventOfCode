#include <stdio.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        char a, b;
        fscanf(fp, "%c %c\n", &a, &b);

        if(a - 'A' == b - 'X'){
            total += 3;
        }
        else if(a == 'A' && b == 'Y' ||
                a == 'B' && b == 'Z' ||
                a == 'C' && b == 'X'){
            total += 6;
        }

        total += b - 'X' + 1;
    }

    printf("%d\n", total);
    return 0;
}