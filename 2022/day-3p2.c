#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int calc_idx(char c){
    if('a' <= c && c <= 'z')
        return c - 'a';
    return c - 'A' + 26;
}

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        int val[53] = {0};

        for(int i=0; i<3; i++){
            bool flag[53] = {false};
            char a[100];
            fscanf(fp, "%s\n", a);
            for(int j=0; j<strlen(a); j++){
                if(flag[calc_idx(a[j])])
                    continue;
                val[calc_idx(a[j])]++;
                flag[calc_idx(a[j])] = true;
            }
        }

        for(int i=0; i<=52; i++){
            if(val[i] == 3){
                total += i + 1;
                break;
            }
        }
    }

    printf("%d\n", total);
    return 0;
}