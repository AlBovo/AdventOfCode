import sys
t = sys.stdin.readline().strip().split(',')
hashmap = [{} for _ in range(256)]
pos = {}
c = 0
# TODO: sort the hashmap when - found
for i in t:
    if '-' in i:
        if i[:-1] in pos.keys():
            hashmap[pos[i[:-1]]].pop(i[:-1], None)
        c += 1
    else:
        v = int(i.split('=')[1])
        val = i.split('=')[0]

        if val in pos.keys():
            hashmap[pos[val]] = v
        else:
            hashmap[c][val] = v
            pos[val] = c

print(hashmap)