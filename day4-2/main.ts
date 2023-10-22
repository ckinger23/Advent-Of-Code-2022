import fs from "fs";

const loopThroughContent = (lines : string[]): number => {
    let totalScore = 0;
    lines.forEach((line) => {
        let pairs = line.split(',');
        let e1_v1 = parseInt(pairs[0].split('-')[0]);
        let e1_v2 = parseInt(pairs[0].split('-')[1]);
        let e2_v1 = parseInt(pairs[1].split('-')[0]);
        let e2_v2 = parseInt(pairs[1].split('-')[1]);
        if (e1_v1 <= e2_v1 && e2_v1 <= e1_v2) {
            console.log(`Option 1: ${ line }`);
            totalScore += 1;
        } else if (e2_v1 <= e1_v1 && e1_v1 <= e2_v2) {
            console.log(`Option 2: ${ line }`);
            totalScore += 1;
        }
    })
    return totalScore;
}

const main = () => {
    const content = fs.readFileSync(process.argv[2], "utf-8");
    const lines = content.split("\n");
    let totalScore = loopThroughContent(lines);
    console.log(totalScore);
}

main();