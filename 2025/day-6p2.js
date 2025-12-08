// this solution assumes that there are no 0s in the input
var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0n;

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let lines = data.split('\n');
        let operators = Array.from(lines[lines.length-1].replaceAll(' ', ''))
        let sizes = [];

        let last = 0;
        for (let i = 1; i < operators.length; i++) {
            let pos = lines[lines.length-1].indexOf(operators[i], last+1);
            sizes.push(pos - last - 1);
            last = pos;
        }
        sizes.push(lines[lines.length-1].length - last);
        
        let nums = [];

        for (let i = 0; i < operators.length; i++) {
            let c = [];
            for (let j = 0; j < lines.length - 1; j++) {
                c.push(lines[j].substring(0, sizes[i]));
                lines[j] = lines[j].substring(sizes[i]+ 1);
            }
            nums.push(c);
        }

        for (let i = 0; i < nums.length; i++) {
            let n = [];
            for (let j = 0; j < sizes[i]; j++) {
                let t = "";
                for (let ns of nums[i]) {
                    t += ns[j];
                }
                n.push(BigInt(t.replaceAll(' ', '')));
            }
            
            if (operators[i] == '+')
                total += n.reduce((a, b) => a + b);
            else
                total += n.reduce((a, b) => a * b);
        }

        console.log(total);
    } else {
        console.log(err);
    }
});
