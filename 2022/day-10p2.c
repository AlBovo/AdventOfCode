#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(){
    FILE *file = fopen("input.txt", "r");
    
    int n;
    long long x = 1;
    char line[40];
    for(int i=0; !feof(file); i++){
        char command[10];
        fscanf(file, "%s", command);
        if(i > 0 && i % 40 == 0){
            printf("%s\n", line);
        }
        if(abs(x-(i%40)) <= 1){
            line[i % 40] = '#';
        } else {
            line[i % 40] = ' ';
        }

        if(strcmp(command, "addx") == 0){
            i++;
            if(i > 0 && i % 40 == 0){
                printf("%s\n", line);
            }
            if(abs(x-(i%40)) <= 1){
                line[i % 40] = '#';
            } else {
                line[i % 40] = ' ';
            }
            fscanf(file, "%d", &n);
            x += n;
        }
        fgetc(file);
    }
}