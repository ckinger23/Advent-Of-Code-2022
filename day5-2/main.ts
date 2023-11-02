import fs from "fs";

const getStringIndexForColumn = (col: number) : number => {
        return col * 4 - 3;
    }

const loopThroughStacks = (lines : string[]): any => {
    const stacks : {[index: string]: any} = {
        "1": [],
        "2": [],
        "3": [],
        "4": [],
        "5": [],
        "6": [],
        "7": [],
        "8": [],
        "9": [],
    };
    


    for (const line of lines) {
        for (let i = 1; i < 10; i++) {
            const strIndex = getStringIndexForColumn(i);
            console.log({ strIndex });
            if (line[strIndex] !== " ") {
                stacks[i].unshift(line[strIndex]);
                console.log( line[strIndex] );
            }
        }
    } 
    return stacks;
}

const loopThroughMoves = (stacks: string[][], moves: string[]): string => {
    for (let move of moves) {
        const moveArray = move
        .split(" ")
        .filter((element) => !isNaN(Number(element)))
        .map(Number);
        const numMoves = moveArray[0];
        const fromCol = moveArray[1];
        const toCol = moveArray[2];
        const stack = stacks[fromCol];
        const removedCrates = stack.splice(stack.length - numMoves, numMoves);
        stacks[toCol].push(...removedCrates);
    }

    let result = "";
    for (const stack in stacks) {
        const column = stacks[stack];
        const topCrate = column[column.length - 1];
        result += topCrate;
    }
    return result;

}


const main = () => {
    const inputStacks = fs.readFileSync(process.argv[2], "utf-8");
    const inputMoves = fs.readFileSync(process.argv[3], "utf-8");
    const splitStacks = inputStacks.trimEnd().split("\n");
    const splitMoves = inputMoves.trimEnd().split("\n");
    console.log( { splitStacks });
    const stacks = loopThroughStacks(splitStacks);
    console.log( {stacks});
    const makesMoves = loopThroughMoves(stacks, splitMoves);
    console.log(makesMoves);
}

main();
