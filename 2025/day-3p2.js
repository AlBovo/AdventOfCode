var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

let total = BigInt(0);
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let lines = data.split('\n');
        for (let line of lines) {
            line = line.trim();
            if(line.length <= 12) {
                total += BigInt(line);
                continue;
            }
            let [curr, curri] = [0,0];
            for (let i = 0; i < 12; i++) {
                let [maxn, maxi] = [0, 0];
                for (let j = curri; j <= line.length - (12 - i); j++) {
                    if (line[j] - "0" > maxn) {
                        maxn = line[j] - "0";
                        maxi = j;
                    }
                }
                curr = curr * 10 + maxn;
                curri = maxi + 1;
            }

            total +=  BigInt(curr);
        }
        console.log('Result: ', total);
    } else {
        console.log(err);
    }
});
