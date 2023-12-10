import sys, re
m = [i.strip() for i in sys.stdin]

stack = []
todo = []

def calculate():
    global todo, stack
    # print(stack)
    l = []
    
    for i in stack:
        done = False
        for e in todo:
            if e[1] <= i < e[1] + e[2]:
                l.append(i - e[1] + e[0])
                done = True
        if not done:
            l.append(i)
    stack = l
    todo = []

for seed in re.findall(r'\d+', m[0]):
    stack.append(int(seed))

# print(m)

m.append('')
for i in range(m.index("seed-to-soil map:") + 1, len(m)):
    if m[i] == '':
        calculate()
        continue
    
    if not m[i].replace(' ', '').isalnum():
        continue
    
    l = re.findall(r'\d+', m[i])
    assert len(l) == 3
    
    todo += [[int(e) for e in l]]

print(min(stack))