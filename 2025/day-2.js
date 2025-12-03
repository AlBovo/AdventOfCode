var fs = require('fs'),
    path = require('path'),    
    filePath = path.join(__dirname, 'input.txt');

const isInvalid = (id) => {
    const strId = String(id.toString());
    if (strId.length % 2 !== 0) {
        return false;
    }
    return strId.substring(0, strId.length / 2) === strId.substring(strId.length / 2);
}

let total = 0;
fs.readFile(filePath, {encoding: 'ascii'}, function(err,data){
    if (!err) {
        let datas = data.trim().split(',');
        console.log(datas);
        for (const ranges in datas) {
            let range = datas[ranges].split('-');
            let [first, second] = range.map(x => parseInt(x));
            const delta = second - first;
            if (first.toString().length === second.toString().length && first.toString().length % 2 !== 0) {
                continue;
            }
            else if (first.toString().length !== second.toString().length) { // TODO: optimize this logic
                for (let id = first; id <= second; id++) {
                    if (isInvalid(id)) {
                        total += id;
                    }
                }
            }
            else {
                for (let id = first; id <= second; id++) { // TODO: optimize this logic
                    if (isInvalid(id)) {
                        total += id;
                    }
                }
            }
        }        

        console.log('Sum of invalid ids:', total);
    } else {
        console.log(err);
    }
});
