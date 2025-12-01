var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0;
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        var lines = data.split('\n');
        let current = 50;
        for (let line of lines) {
            // console.log(line);
            let a = line[0];
            let b = parseInt(line.substring(1), 10);
            
            current += (a === 'R' ? b : -b);

            if (current < 0) {
                while (current < 0) {
                    current = 100 + current;
                }
            } else if (current > 99) {
                current = current % 100;
            }

            if (current === 0) {
                total += 1;
            }
            // console.log(current);
        }
        console.log('Total times at 0:', total);
    } else {
        console.log(err);
    }
});
