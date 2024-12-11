#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define SIZE 99

int main(){
    FILE *fp = fopen("input.txt", "r");
    int mat[SIZE][SIZE] = {0};
    bool vis[SIZE][SIZE] = {0};

    int x = 0, y = 0;
    while(true){
        char c = fgetc(fp);
        if(c == EOF){
            break;
        }
        if(c == '\n'){
            y++;
            x = 0;
            continue;
        }
        mat[y][x++] = c - '0';
    }

    for(int i=0; i<SIZE; i++){
        int maxiX = -1, maxiY = -1;
        for(int j=0; j<SIZE; j++){
            if(mat[i][j] > maxiX){
                maxiX = mat[i][j];
                vis[i][j] = true;
            }
            if(mat[j][i] > maxiY){
                maxiY = mat[j][i];
                vis[j][i] = true;
            }
        }
        
        maxiX = -1; maxiY = -1;

        for(int j=SIZE-1; j>=0; j--){
            if(mat[i][j] > maxiX){
                maxiX = mat[i][j];
                vis[i][j] = true;
            }
            if(mat[j][i] > maxiY){
                maxiY = mat[j][i];
                vis[j][i] = true;
            }
        }
    }

    int somma = 0;
    for(int i=0; i<SIZE; i++){
        for(int j=0; j<SIZE; j++){
            if(vis[i][j]){
                somma += vis[i][j];
            }
        }
    }

    printf("%d\n", somma);
    return 0;
}