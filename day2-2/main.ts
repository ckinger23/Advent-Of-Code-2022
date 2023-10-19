import fs from "fs";

const content = fs.readFileSync(process.argv[2], "utf-8");
console.log(process.argv[2]);

const lines = content.split("\n");
let totalScore = 0;

lines.forEach((line) => {
  const choices = line.split(" ");
  const opposition = choices[0];
  const myPick = choices[1];
  switch (opposition) {
    case "A":
      switch (myPick) {
        case "X":
          totalScore += 3;
          break;
        case "Y":
          totalScore += 4;
          break;
        case "Z":
          totalScore += 8;
          break;
        default:
          console.log("Incorrect personal choice");
      }
      break;
    case "B":
      switch (myPick) {
        case "X":
          totalScore += 1;
          break;
        case "Y":
          totalScore += 5;
          break;
        case "Z":
          totalScore += 9;
          break;
        default:
          console.log("Incorrect personal choice");
      }
      break;
    case "C":
      switch (myPick) {
        case "X":
          totalScore += 2;
          break;
        case "Y":
          totalScore += 6;
          break;
        case "Z":
          totalScore += 7;
          break;
        default:
          console.log("Incorrect personal choice");
      }
      break;
    default:
      console.log("Incorrect choice from the opposition");
  }
});

console.log(totalScore);