var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

const isInvalid = (id) => {
    const strId = String(id.toString());
    let sum = BigInt(0);
    for (let i = 1; i <= strId.length; i++) {
        if (strId.length % i !== 0) {
            continue;
        }
        for (let j = i; j < strId.length; j += i) {
            if (strId.substring(0, i) !== strId.substring(j, j + i)) {
                break;
            }
            if (j + i === strId.length) {
                return BigInt(id);
            }
        }
    }
    return sum;
}

let total = BigInt(0);
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let datas = data.trim().split(',');
        console.log(datas);
        for (const ranges in datas) {
            let range = datas[ranges].split('-');
            let [first, second] = range.map(x => BigInt(x));
            for (let id = first; id <= second; id++) { // TODO: optimize this logic
                let v = isInvalid(id);
                total += v;
            }
        }

        console.log('Sum of invalid ids:', total);
    } else {
        console.log(err);
    }
});
