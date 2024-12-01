#include <stdio.h>

int main(){
    FILE *fp = fopen("input.txt", "r");

    int maxi[3] = {-1, -1, -1}, tot = 0;
    while(!feof(fp)){
        int x; char t;
        if((t = fgetc(fp)) != '\n'){
            ungetc(t, fp);
            fscanf(fp, "%d", &x);
            fgetc(fp);
            tot += x;
        }
        else{
            for(int i=0; i<3; i++){
                if(maxi[i] == -1 || tot > maxi[i]){
                    for(int j=2; j>i; j--){
                        maxi[j] = maxi[j-1];
                    }
                    maxi[i] = tot;
                    break;
                }
            }
            tot = 0;
        }
    }
    printf("%d\n", maxi[0] + maxi[1] + maxi[2]);
    return 0;
}