import sys
t = [i.strip() for i in sys.stdin]
rowsm = [[]]

curr = 0
for i in t:
    if i == "":
        curr += 1
        rowsm.append([])
        continue
    rowsm[curr].append(i)

colsm = [[] for _ in range(len(rowsm))]
for i in range(len(rowsm)):
    for j in range(len(rowsm[i][0])):
        col = "".join([rowsm[i][k][j] for k in range(len(rowsm[i]))])
        colsm[i].append(col)


rows, cols = 0, 0
for i in range(len(rowsm)):
    for j in range(1, len(rowsm[i])):
        row1 = rowsm[i][:j]
        row2 = rowsm[i][j:]
        lenm = min(len(row1), len(row2))
        if row1[-lenm:] == row2[:lenm][::-1]:
            rows += j
            break
    for j in range(1, len(colsm[i])):
        col1 = colsm[i][:j]
        col2 = colsm[i][j:]
        lenm = min(len(col1), len(col2))
        if col1[-lenm:] == col2[:lenm][::-1]:
            cols += j
            break
print(rows*100+cols)