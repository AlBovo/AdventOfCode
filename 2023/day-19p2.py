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


pos = "in"
while pos not in ['A', 'R']:
    for rule in workflow[pos]:
        if str(rule).count(':') == 0:
            pos = rule
            break
        else:
            rule = rule.split(':')
            if '<' in rule[0]:
                val = int(rule[0][rule[0].index('<')+1:])
            else:
                val = int(rule[0][rule[0].index('>')+1:])
            print(val)