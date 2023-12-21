import sys, json

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

ratings = []
for i in sys.stdin:
    f = json.loads(clear(i.strip()))
    ratings.append(f)

tot = 0
for e in ratings:
    pos = "in"
    x, m, a, s = e['x'], e['m'], e['a'], e['s']
    while pos not in ['A', 'R']:
        for rule in workflow[pos]:
            if str(rule).count(':') == 0:
                pos = rule
                break
            else:
                rule = rule.split(':')
                if eval(rule[0]):
                    pos = rule[1]
                    break
    if pos == 'A':
        tot += x + m + a + s
print(tot)
                    