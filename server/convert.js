import { readFileSync, writeFileSync } from "node:fs";

const FILTERED_STRINGS = ["", "Teksting 2024"];

const content = readFileSync("subs/raw.txt").toString().replaceAll("\r", "").split("\n");

let output = [];
for (const line of content) {
  let newLine = line.trim();
  if (FILTERED_STRINGS.includes(newLine)) {
    continue;
  }
  if (newLine.toUpperCase() === newLine) {
    newLine = `# ${newLine}`;
  }
  newLine = newLine.replaceAll("/", "\\n");
  output.push(newLine);
}

writeFileSync("subs/undertekster2024.txt", output.join("\n"), function (err) {
  if (err) return console.log(err);
  console.log("File is created successfully.");
});
