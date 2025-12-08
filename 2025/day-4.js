var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0;
var matrix;

function check(x, y) {
    return ( 
        (matrix[x-1][y-1] == '@') + (matrix[x-1][y] == '@') + (matrix[x-1][y+1] == '@') +
        (matrix[x][y-1] == '@') + (matrix[x][y+1] == '@') +
        (matrix[x+1][y-1] == '@') + (matrix[x+1][y] == '@') + (matrix[x+1][y+1] == '@')
    ) < 4;
}

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        matrix = data.split('\n').map(row => Array.from(row));
        matrix = [Array.from({"length":matrix[0].length+2}, () => '.')].concat(
                matrix.map((row) => ['.'].concat(row, ['.'])),
                [Array.from({"length":matrix[0].length+2}, () => '.')]
        );
        for (let i = 0; i < matrix.length; i++) {
            for (let j = 0; j < matrix[0].length; j++) {
                if (matrix[i][j] == '@') {
                    total += check(i, j);
                }
            }
        }
        // console.log((matrix.map(row => row + "\n")).join())
        console.log(total);
    } else {
        console.log(err);
    }
});
