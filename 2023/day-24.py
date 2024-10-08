import numpy as np
import sys as sus
import math

m1 = [[[int(_) for _ in e.strip().split(',')] for e in i.split('@')] for i in sus.stdin]
lines = []

LEAST = 200000000000000
MOST = 400000000000000

for line in m1:
    x1, y1, z1 = line[0]
    x2, y2, z2 = line[0]
    x2 += line[1][0]
    y2 += line[1][1]
    z2 += line[1][2]
    
    a = y2 - y1
    b = x1 - x2
    c = x2 * y1 - x1 * y2
    lines.append((a, b, c))


def intersezione_rette(A1, B1, C1, A2, B2, C2):
    matrice = np.array([[A1, B1], [A2, B2]])
    termini_not = np.array([-C1, -C2])
    det = np.linalg.det(matrice)
    
    if det == 0:
        return None

    punto_intersezione = np.linalg.solve(matrice, termini_not)
    return punto_intersezione

tot = 0
for i, line1 in enumerate(lines):
    for e, line2 in enumerate(lines):
        if e <= i:
            continue
        
        a1, b1, c1 = line1
        a2, b2, c2 = line2
                
        punto = intersezione_rette(a1, b1, c1, a2, b2, c2)
        
        if punto is not None:
            if not (LEAST <= punto[0] <= MOST and LEAST <= punto[1] <= MOST):
                continue
            
            x1 = m1[i][0][0]
            x2 = m1[i][0][0] + m1[i][1][0]         
            
            if x1 < x2 and punto[0] < x1:
                continue
            
            if x1 > x2 and punto[0] > x1:
                continue
            
            x1 = m1[e][0][0]
            x2 = m1[e][0][0] + m1[e][1][0]        

            if x1 < x2 and punto[0] < x1:
                continue
            
            if x1 > x2 and punto[0] > x1:
                continue
            
            print(i, e, punto, f"{(a1, b1, c1)},{(a2, b2, c2)}")
            tot += 1
                
print(tot)
