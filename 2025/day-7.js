const { assert } = require('console');

var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0n;

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let matrix = data.split('\n').map(row => Array.from(row));
        let vis = new Set();
        let pos = [[0, Math.floor(matrix[0].length / 2)]];
        assert(matrix[pos[0][0]][pos[0][1]] == 'S');
        while (pos.length > 0) {
            var [y, x] = pos.pop();
            if (y >= matrix.length) continue;
            // bruhhh
            if (vis.has([y, x].toString())) continue;
            vis.add([y, x].toString());

            if (matrix[y][x] == '^') {
                total++;
                pos.push([y+1, x-1]);
                pos.push([y+1, x+1]);
            }
            else {
                pos.push([y+1, x]);
            }
        }

        console.log(total);
    } else {
        console.log(err);
    }
});
