var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = 0;

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let lines = data.split('\n');
        let ranges = [];
        
        let i = 0;
        while (lines[i] != '') {
            ranges.push(lines[i].split('-').map(x => parseInt(x)))
            i++;
        }

        i++;
        while (i < lines.length) {
            let id = parseInt(lines[i]);
            for (let r = 0; r < ranges.length; r++) {
                let [x, y] = ranges[r];
                if (x <= id && id <= y)  {
                    total++;
                    break;
                }
            }
            i++;
        }

        console.log(total);
    } else {
        console.log(err);
    }
});
