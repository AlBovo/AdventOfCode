import sys
t = sys.stdin.readline().strip().split(',')
hashmap = [{} for _ in range(256)]
pos = {}

def HASH(st: str):
    r = 0
    for e in st:
        r = ((r + ord(e)) * 17) % 256
    return r

for i in t:
    if '-' in i:
        if i[:-1] not in pos.keys():
            continue
        hashmap[pos[i[:-1]]].pop(i[:-1])
        pos.pop(i[:-1])
    else:
        val = int(i.split('=')[1])
        v = i.split('=')[0]
        if i[:-1] not in pos.keys():
            pos[v] = HASH(v)
        hashmap[pos[v]][v] = val
sum = 0
for i in pos:
    v = pos[i]
    r = (v+1) * (list(hashmap[v].keys()).index(i) + 1) * hashmap[v][i]
    sum += r
print(sum)