const { assert } = require('console');

var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0n;

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let matrix = data.split('\n').map(row => Array.from(row));
        let enc = [];

        for (let i=0; i<matrix.length; i++){
            let found = [];
            for (let j=0; j<matrix[i].length; j++) {
                if (matrix[i][j] == '^')
                    found.push(j);
            }
            enc.push(found);
        }

        let pos = new Map();
        pos.set(Math.floor(matrix[0].length / 2), 1);
        
        for (let i=1; i<enc.length; i++) {
            let c = new Map();
            for (const t of enc[i]) {
                if (pos.has(t)) {
                    c.set(t-1, (c.get(t-1) || 0) + (pos.get(t-1) || 0) + pos.get(t));
                    c.set(t+1, (c.get(t+1) || 0) + (pos.get(t+1) || 0) + pos.get(t));
                    pos.delete(t);
                    pos.delete(t-1);
                    pos.delete(t+1);
                }
            }
            pos = new Map([...pos, ...c]);
        }

        for (const v of pos.values()) {
            total += BigInt(v);
        }

        console.log(total);
    } else {
        console.log(err);
    }
});
