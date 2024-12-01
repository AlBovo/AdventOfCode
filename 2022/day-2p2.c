#include <stdio.h>

int mod_neg(int a, int b){
    return (a % b + b) % b;
}

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        char a, b;
        fscanf(fp, "%c %c\n", &a, &b);

        switch(b){
            case 'X':
                b = mod_neg(a - 'A' - 1, 3) + 'X';
                break;
            case 'Y':
                b = a - 'A' + 'X';
                break;
            case 'Z':
                b = mod_neg(a - 'A' + 1, 3) + 'X';
                break;
        }


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