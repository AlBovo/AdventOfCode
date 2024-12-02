#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool check(char *str){
    for(int i=0; i<14; i++){
        for(int e=0; e<i; e++)
            if(str[i] == str[e])
                return false;
    }
    return true;
}

int main(){
    FILE *fp = fopen("input.txt", "r");
    char str[5000];
    fscanf(fp, "%s", str);

    char *ptr = str;
    for(int i=14; i<strlen(str); i++, ptr++){
        if(check(ptr)){
            printf("%d\n", i);
            return 0;
        }
    }    
    return 0;
}