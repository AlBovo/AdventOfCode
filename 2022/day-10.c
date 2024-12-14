#include <stdio.h>
#include <string.h>

int main(){
    FILE *file = fopen("input.txt", "r");
    
    int n, i = 1;
    long long total = 0, x = 1;
    for(; !feof(file); i++){
        char command[10];
        fscanf(file, "%s", command);

        if(i >= 20 && (i-20) % 40 == 0){
            total += i * x;
            printf("%d %lld\n", i, x);
        }

        if(strcmp(command, "addx") == 0){
            i++;
            if(i >= 20 && (i-20) % 40 == 0){
                total += i * x;
                printf("%d %lld\n", i, x);
            }
            fscanf(file, "%d", &n);
            x += n;
        }
        fgetc(file);
    }
    printf("%lld\n", total);
}