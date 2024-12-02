#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX 500
#define SIZE 9

typedef struct {
    int top;
    unsigned capacity;
    char* array;
} Stack;

Stack* createStack(unsigned capacity){
    Stack* stack = (Stack*)malloc(sizeof(Stack));
    stack->capacity = capacity;
    stack->top = -1;
    stack->array = (char*)malloc(stack->capacity);
    return stack;
}

int isFull(Stack* stack){
    return stack->top == stack->capacity - 1;
}

int isEmpty(Stack* stack){
    return stack->top == -1;
}

void push(Stack* stack, char item){
    if (isFull(stack))
        return;
    stack->array[++stack->top] = item;
    // printf("%c pushed to stack\n", item);
}

char pop(Stack* stack){
    if (isEmpty(stack))
        return -1;
    return stack->array[stack->top--];
}

int main(){
    FILE *fp = fopen("input.txt", "r");

    Stack *s[SIZE];
    for(int i=0; i<SIZE; i++){
        s[i] = createStack(MAX);
    }    

    int total = 0;
    while(!feof(fp)){
        bool flag = false;
        while(!flag){
            char c[4*SIZE] = {0};
            fscanf(fp, "%[^\n]s", c);
            for(int i=0; i<SIZE; i++){
                if(flag = (c[i*4+1] == i + '1'))
                    break;
                if(c[i*4+1] == ' ')
                    continue;
                push(s[i], c[i*4+1]);   
            }
            fgetc(fp); // consume newline
        }

        // gotta reverse all the stacks :(
        for(int i=0; i<SIZE; i++){
            char temp[MAX];
            int j = 0;
            while(!isEmpty(s[i])){
                temp[j++] = pop(s[i]);
            }
            for(int k=0; k<j; k++){
                push(s[i], temp[k]);
            }
        }

        fgetc(fp);
        while(!feof(fp)){
            int a, b, c;
            fscanf(fp, "move %d from %d to %d\n", &a, &b, &c);
            char temp[a];
            for(int i=0; i<a; i++)
                temp[i] = pop(s[b-1]);
            for(int i=a-1; i>=0; i--)
                push(s[c-1], temp[i]);
        }
    }

    for(int i=0; i<SIZE; i++)
        printf("%c", pop(s[i]));

    for(int i=0; i<SIZE; i++)
        free(s[i]);

    return 0;
}