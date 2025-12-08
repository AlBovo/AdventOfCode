var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var matrix;
var countmat;

function count(x, y) {
    return ( 
        (matrix[x-1][y-1] == '@') + (matrix[x-1][y] == '@') + (matrix[x-1][y+1] == '@') +
        (matrix[x][y-1] == '@') + (matrix[x][y+1] == '@') +
        (matrix[x+1][y-1] == '@') + (matrix[x+1][y] == '@') + (matrix[x+1][y+1] == '@')
    );
}

function reduce(x, y) {
    countmat[x][y] = 1e9;
    for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
            if (matrix[x+i][y+j] == '@') {
                countmat[x+i][y+j]--;
            }
        }
    }
}

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        matrix = data.split('\n').map(row => Array.from(row));
        matrix = [Array.from({"length":matrix[0].length+2}, () => '.')].concat(
                matrix.map((row) => ['.'].concat(row, ['.'])),
                [Array.from({"length":matrix[0].length+2}, () => '.')]
        );

        countmat = JSON.parse(JSON.stringify(matrix));
        let pos = [];
        for (let i = 0; i < matrix.length; i++) {
            for (let j = 0; j < matrix[0].length; j++) {
                if (matrix[i][j] == '@') {
                    countmat[i][j] = count(i, j);
                    pos.push([countmat[i][j], [i, j]]);
                }
            }
        }
        
        let counts = 0;
        while (true) {
            let ccopy = counts;
            pos.sort();

            for (let p = 0; p < pos.length; p++) {
                let [x, y] = pos[p][1];
                if (countmat[x][y] < 4) {
                    reduce(x, y);
                    counts++;
                    pos[p][0] = 1e9;
                }
            }

            if (ccopy == counts) {
                break;
            }
        }
        
        // console.log((matrix.map(row => row + "\n")).join());
        console.log(counts);
    } else {
        console.log(err);
    }
});
