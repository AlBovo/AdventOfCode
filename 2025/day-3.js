var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

let total = 0;
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let lines = data.split('\n');
        for (let line of lines) {
            line = line.trim();
            if(line.length === 1) {
                total += parseInt(line);
                continue;
            }
            let [maxn, maxi] = [0, -1];
            for (let i = 0; i < line.length - 1; i++) {
                if (line[i] - "0" > maxn) {
                    maxn = line[i] - "0";
                    maxi = i;
                }
            }
            let [minn, mini] = [0, -1];
            for (let i = maxi+1; i < line.length; i++) {
                if (line[i] - "0" > minn) {
                    minn = line[i] - "0";
                    mini = i;
                }
            }
            // console.log(maxn * 10 + minn)
            total +=  maxn * 10 + minn;
        }
        console.log('Result: ', total);
    } else {
        console.log(err);
    }
});
