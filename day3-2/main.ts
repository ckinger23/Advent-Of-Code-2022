import fs from "fs";
const findTheMatch = (
    firstSack: string,
    secondSack: string,
    thirdSack: string
): string => {
    for (const character of firstSack) {
        if (secondSack.includes(character) && thirdSack.includes(character)) {
            return character;
        }
    }
    return "no match"
}

const letterScore = (
    matchChar: string
): number => {
    const character = matchChar.charAt(0);
    console.log(character);
    if (character == character.toUpperCase()) {
        return matchChar.charCodeAt(0) - 38;
    } else {
        return matchChar.charCodeAt(0) - 96;
    }
}


const content = fs.readFileSync(process.argv[2], "utf-8");

const lines = content.split("\n");
let totalScore = 0;

for (let i = 2; i < lines.length; i+=3) {
    const firstSack = lines[i -2];
    const secondSack = lines[i-1];
    const thirdSack = lines[i];
    const match = findTheMatch(firstSack, secondSack, thirdSack);
    const charScore = letterScore(match);
    totalScore += charScore;
}

console.log(totalScore);

