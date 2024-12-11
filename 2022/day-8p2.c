#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define SIZE 99

long long max(long long a, long long b){
    return a > b ? a : b;
}

long long min(long long a, long long b){
    return a < b ? a : b;
}

int main(){
    FILE *fp = fopen("input.txt", "r");
    int mat[SIZE][SIZE] = {0};

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

    long long maxi = -1;
    for(int i=0; i<SIZE; i++){
        for(int j=0; j<SIZE; j++){
            long long total = 1;
            
            int x, y;
            x = i - 1; y = j;
            while(x >= 0 && mat[x][y] < mat[i][j]){
                x--;
            }
            total *= i == 0 ? 0 : abs(max(x, 0) - i);

            x = i + 1; y = j;
            while(x < SIZE && mat[x][y] < mat[i][j]){
                x++;
            }
            total *= i == SIZE-1 ? 0 : abs(min(x, SIZE-1) - i);

            x = i; y = j - 1;
            while(y >= 0 && mat[x][y] < mat[i][j]){
                y--;
            }
            total *= j == 0 ? 0 : abs(max(y, 0) - j);

            x = i; y = j + 1;
            while(y < SIZE && mat[x][y] < mat[i][j]){
                y++;
            }
            total *= j == SIZE-1 ? 0 : abs(min(y, SIZE-1) - j);

            maxi = maxi > total ? maxi : total;
        }
    }
    

    printf("%lld\n", maxi);
    return 0;
}