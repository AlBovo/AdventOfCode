var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

var total = BigInt(0);

fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let lines = data.split('\n');
        let ranges = [];
        
        let i = 0;
        while (lines[i] != '') {
            ranges.push(lines[i].split('-').map(x => BigInt(x)))
            i++;
        }

        ranges.sort((a, b) => {
            if (a[0] < b[0]) return -1;
            if (a[0] > b[0]) return 1;
            return 0;
        });

        let merged = [];
        if (ranges.length > 0) {
            merged.push(ranges[0]);
            for (let i = 1; i < ranges.length; i++) {
                let last = merged[merged.length - 1];
                let current = ranges[i];
                
                if (current[0] <= last[1] + 1n) {
                    if (current[1] > last[1]) {
                        last[1] = current[1];
                    }
                } else {
                    merged.push(current);
                }
            }
        }

        for (let range of merged) {
            total += range[1] - range[0] + 1n;
        }
        
        console.log(total);
    } else {
        console.log(err);
    }
});
