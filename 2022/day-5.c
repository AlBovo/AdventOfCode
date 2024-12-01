#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX 200

typedef struct {
    char data[MAX];
    int top;
} Stack;

void initStack(Stack *s) {
    s->top = -1;
}

int isFull(Stack *s) {
    return s->top == MAX - 1;
}

int isEmpty(Stack *s) {
    return s->top == -1;
}

void push(Stack *s, char value) {
    if (isFull(s)) {
        printf("Stack overflow\n");
        return;
    }
    s->data[++(s->top)] = value;
}

char pop(Stack *s) {
    if (isEmpty(s)) {
        printf("Stack underflow\n");
        return -1;
    }
    return s->data[(s->top)--];
}

int main(){
    FILE *fp = fopen("input.txt", "r");

    Stack* s[9];
    for(int i=0; i<9; i++){
        s[i] = malloc(sizeof(Stack));
        initStack(s[i]);
    }    

    int total = 0;
    while(!feof(fp)){
        bool flag = false;
        while(!flag){
            char c[35] = {0};
            fscanf(fp, "%[^\n]s", c);
            for(int i=0; i<9; i++){
                if(flag = (c[i*4+1] == i + '1'))
                    break;
                if(c[i*4+1] == ' ')
                    continue;
                push(s[i], c[i*4+1]);   
            }
            fgetc(fp); // consume newline
        }
        fgetc(fp);
        while(!feof(fp)){
            int a, b, c;
            fscanf(fp, "move %d from %d to %d\n", &a, &b, &c);
            for(int i=0; i<a; i++){
                push(s[c-1], pop(s[b-1]));
            }
        }
    }

    for(int i=0; i<9; i++)
        printf("%c", pop(s[i]));

    for(int i=0; i<9; i++)
        free(s[i]);

    // TODO: fix bugs 
    return 0;
}