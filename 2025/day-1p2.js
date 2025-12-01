var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0;
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        var lines = data.split('\n');
        let current = 50;
        for (let line of lines) {
            let dir = line[0];
            let dist = parseInt(line.substring(1), 10);
            let delta = dir === 'R' ? dist : -dist;
            let next = current + delta;

            if (delta > 0) {
                total += Math.floor(next / 100) - Math.floor(current / 100);
            } else if (delta < 0) {
                total += Math.ceil(current / 100) - Math.ceil(next / 100);
            }

            current = next;
        }
        console.log(total);
    } else {
        console.log(err);
    }
});
