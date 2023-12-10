import sys
m = [i.strip().split() for i in sys.stdin]
cards = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'][::-1]
# 6: Five of a kind
# 5: Four of a kind
# 4: Full house
# 3: Three of a kind
# 2: Two pair
# 1: One pair
# 0: High card

def calculate(val: list):
    if val.count(val[0]) == 5:
        return 6
    t = []
    done = [0]
    js = val.count(0)
    for i in val:
        if i in done:
            continue
        t.append(val.count(i))
        done.append(i)
    t.sort(reverse=True)
    t[0] += js
    print("".join([cards[i] for i in val]), t)
    
    if 5 in t:
        return 6
    if 4 in t:
        return 5
    if 3 in t and 2 in t:
        return 4
    if 3 in t:
        return 3
    if t.count(2) == 2:
        return 2
    if 2 in t:
        return 1
    return 0
    
tot = 0
winning = [[] for _ in range(7)]
for i in range(len(m)):
    m[i][1] = int(m[i][1])
    m[i][0] = [cards.index(e) for e in m[i][0]]
    v = calculate(sorted(m[i][0]))
    m[i][0] = b"".join([bytes([_]) for _ in m[i][0]])
    winning[v].append(m[i])

c = 1
print(winning)
for i in winning:
    i.sort(key=lambda x: x[0])
    for e in i:
        tot += e[1] * c
        c += 1
print(tot)