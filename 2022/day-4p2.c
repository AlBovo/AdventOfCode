#include <stdio.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        int a, b, c, d;
        fscanf(fp, "%d-%d,%d-%d\n", &a, &b, &c, &d);

        if(a <= c && c <= b)
            total++;
        else if(a <= d && d <= b)
            total++;
        else if(c <= a && a <= d)
            total++;
        else if(c <= b && b <= d)
            total++;
    }

    printf("%d\n", total);
    return 0;
}