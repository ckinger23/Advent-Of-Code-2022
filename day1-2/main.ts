import fs from "fs";

const content = fs.readFileSync(process.argv[2], "utf-8");
console.log(process.argv[2]);

const lines = content.split("\n");

let count = 0;
const totals : number[] = [];

lines.forEach((line) => {
    if (line == '') {
        totals.push(count);
        count = 0;
    } else {
        count += Number(line);
    }
});

totals.sort((a, b) => a - b).reverse();
console.log(totals);
const sum = totals[0] + totals[1] + totals[2];
console.log(sum);