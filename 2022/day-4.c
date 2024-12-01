#include <stdio.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        int a, b, c, d;
        fscanf(fp, "%d-%d,%d-%d\n", &a, &b, &c, &d);

        if(a <= c && d <= b || c <= a && b <= d){
            total++;
        }
    }

    printf("%d\n", total);
    return 0;
}