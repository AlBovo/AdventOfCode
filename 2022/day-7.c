#include <stdio.h>
#include <string.h>
#include <assert.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_FILES 40
const int SIZE = 100000;

typedef struct {
    char name[25];
    int size;
} File;

typedef struct {
    char name[25];
    void **file; // files
    void **dir; // directories
    
    int files, dirs;
    void *parent; // cast to Directory
    int size;
} Directory;

Directory* initDirectory(char *name, Directory *parent){
    Directory *dir = (Directory *)malloc(sizeof(Directory));

    strcpy(dir->name, name);
    dir->size = 0;
    dir->parent = parent;
    dir->files = 0;
    dir->dirs = 0;
    dir->file = malloc(MAX_FILES * sizeof(File *));
    dir->dir = malloc(MAX_FILES * sizeof(Directory *));

    return dir;
}

File* addFile(Directory *dir, char *name, int size){
    File *file = (File *)malloc(sizeof(File));
    strcpy(file->name, name);
    file->size = size;

    dir->file[dir->files++] = file;
    dir->size += size;
    return file;
}

Directory* addDirectory(Directory *dir, char *name){
    Directory *newDir = initDirectory(name, dir);
    dir->dir[dir->dirs++] = newDir;
    return newDir;
}

bool fileExists(Directory *dir, char *name){
    for(int i = 0; i < dir->files; i++){
        File *file = (File *)dir->file[i];
        if(strcmp(file->name, name) == 0){
            return true;
        }
    }

    return false;
}

Directory* findDirectory(Directory *dir, char *name){
    for(int i = 0; i < dir->dirs; i++){
        Directory *directory = (Directory *)dir->dir[i];
        if(strcmp(directory->name, name) == 0){
            return directory;
        }
    }

    return NULL;
}

#define DEBUG
int tot = 0;

int dfs(Directory *dir){
    int total = dir->size;
    for(int i = 0; i < dir->dirs; i++){
        Directory *directory = (Directory *)dir->dir[i];
        total += dfs(directory);
    }
    
    if(total < SIZE)
        tot += total;

    return total;
}

int main(){
    FILE *fp = fopen("input.txt", "r");
    Directory *root = initDirectory("/", NULL);
    Directory *current = root;
    while(fgetc(fp) != '\n') continue; // remove first $ cd /

    while(!feof(fp)){
        char c[25] = {0};
        fscanf(fp, "%[^\n]s", c);

        if(strncmp(c, "$ cd ..", 7) == 0){
            assert(current->parent != NULL);
            current = (Directory *)current->parent;
            #ifdef DEBUG
                printf("New current directory: %s\n", current->name);
                puts("Parent directory");
            #endif
        }
        else if(strncmp(c, "$ cd ", 5) == 0){
            if(findDirectory(current, c+5) == NULL){
                current = addDirectory(current, c+5);
            }
            else{
                current = findDirectory(current, c+5);
                assert(current != NULL);
            }
            #ifdef DEBUG
                printf("Current directory: %s\n", current->name);
                printf("Going to directory %s\n", c+5);
            #endif
        }
        else if(strncmp(c, "$ ls", 4) == 0){
            while(!feof(fp)){
                fgetc(fp);
                fscanf(fp, "%[^\n]s", c);
                if(c[0] == '$'){
                    for(int i = strlen(c)-1; i >= 0; i--)
                        ungetc(c[i], fp);
                    ungetc('\n', fp);
                    break;
                }

                char *t = strtok(c, " ");
                if(strcmp(t, "dir") == 0){
                    char *name = strtok(NULL, " ");
                    if(findDirectory(current, name) == NULL){
                        addDirectory(current, name);
                    }
                }
                else{
                    char *name = strtok(NULL, " ");
                    if(name == NULL) continue;
                    if(!fileExists(current, name)){
                        addFile(current, name, atoi(t));
                    }
                }
            }

            #ifdef DEBUG
                printf("Current directory: %s\n", current->name);
                puts("Listing directory");
            #endif
            
        }
        else
            puts("Invalid command");
        
        fgetc(fp);
    }
    dfs(root);

    printf("Total size: %d\n", tot);
    return 0;
}