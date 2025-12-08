var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0n;

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let curr;
        while (true) {
            curr = data;
            data = data.replaceAll('  ', ' ');
            if (curr == data) break;
        }
        let lines = data.split('\n');
        let ops = [];
        let idx = 0;
        while (!lines[idx].includes("+") && !lines[idx].includes("*")) {
            ops.push(lines[idx].trim().split(' '));
            idx++;
        }
        let operators = lines[idx].trim().split(' ');

        for (let i = 0; i < operators.length; i++) {
            let nums = [];
            for (idx = 0; idx < ops.length; idx++) {
                nums.push(BigInt(ops[idx][i]));
            }
            if (operators[i] == '+')
                total += nums.reduce((a, b) => a + b);
            else
                total += nums.reduce((a, b) => a * b);
        }
        console.log(total);
    } else {
        console.log(err);
    }
});
