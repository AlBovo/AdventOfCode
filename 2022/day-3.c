#include <stdio.h>
#include <string.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int total = 0;
    while(!feof(fp)){
        char a[100];
        fscanf(fp, "%s\n", &a);

        int maxi = -1; char most;
        for(int i=0; i<strlen(a)/2; i++){
            int tot = 0;
            for(int e=strlen(a)/2; e<strlen(a); e++){
                if(a[i] == a[e]){
                    tot++;
                }
            }

            if(tot > maxi){
                maxi = tot;
                most = a[i];
            }
        }

        if('a' <= most && most <= 'z'){
            total += most - 'a' + 1;
        }
        else{
            total += most - 'A' + 27;
        }
    }

    printf("%d\n", total);
    return 0;
}