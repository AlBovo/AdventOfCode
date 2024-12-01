#include <stdio.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int maxi = -1, tot = 0;
    while(!feof(fp)){
        int x; char t;
        if((t = fgetc(fp)) != '\n'){
            ungetc(t, fp);
            fscanf(fp, "%d", &x);
            fgetc(fp);
            tot += x;
        }
        else{
            maxi = maxi < tot ? tot : maxi;
            tot = 0;
        }
    }
    printf("%d\n", maxi);
    return 0;
}