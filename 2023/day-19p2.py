import sys

workflow = {}
for i in sys.stdin:
    i = i.strip()

    if i == "":
        break

    name = i[:i.index('{')]
    value = i[i.index('{'):].replace("{", "").replace("}", "").split(',')
    workflow[name] = value

def clear(v: str):
    v = v.replace("x", '"x"')
    v = v.replace("m", '"m"')
    v = v.replace("a", '"a"')
    v = v.replace("s", '"s"')
    v = v.replace("=", ":")
    return v

endRanges = []
pos = [("in", {i: (1, 4000) for i in "xmas"})]
while len(pos) > 0:
    p = pos[0][0]
    ranges = pos[0][1]
    pos = pos[1:]
    if p in ['A', 'R']:
        if p == 'A':
            endRanges.append([i[1] for i in list(ranges.items())])
            # print(ranges)
        continue
    
    for rule in workflow[p]:
        if str(rule).count(':') == 0:
            pos.append((rule, ranges))
        else:
            rule = rule.split(':')
            if '<' in rule[0]:
                letter = rule[0][0]
                val = int(rule[0][rule[0].index('<')+1:])
                ranges2 = ranges.copy()
                ranges2[letter] = (ranges2[letter][0], min(ranges2[letter][1], val))
                pos.append((rule[1], ranges2))
            else:
                letter = rule[0][0]
                val = int(rule[0][rule[0].index('>')+1:])
                ranges2 = ranges.copy()
                ranges2[letter] = (max(ranges2[letter][0], val), ranges2[letter][1])
                pos.append((rule[1], ranges2))

tot = 0
done = []
for i in endRanges:
    r = 1
    if i in done or any(e[0] > e[1] for e in i): # cannot have range [42, 21]
        continue
    print(i)
    done.append(i)
    for e in i:
        r *= e[1] - e[0]
    print(r)
    tot += r # TODO: finish lol
print(tot) # average not my result 167409079868000