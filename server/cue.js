import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import terminalkit from "terminal-kit";
const { terminal: term } = terminalkit;

export async function start(updateText, subs_dir) {
  await new Promise((r) => setTimeout(r, 1000));
  term.fullscreen(true);
  term.grabInput();
  const file = await getFile(subs_dir);
  const lines = getLines(file);
  runSelector(lines, updateText, file);
}

async function getFile(subs_dir) {
  term.cyan("Choose a subtitle:\n");
  const items = readdirSync("subs");
  const file = await term.gridMenu(items).promise;
  return path.join(subs_dir, file.selectedText);
}

function getLines(file) {
  const content = readFileSync(file)
    .toString()
    .replaceAll("\r", "")
    .split("\n")
    .map((t) => t.trim());
  const lines = ["", ...content, ""];
  return lines;
}

async function runSelector(lines, updateText, file) {
  let i = 0;
  let disableSend = false;
  drawLines(lines, i);

  term.on("key", (name, match, data) => {
    if (name == "q" || name == "CTRL_C") {
      // quit
      //term.clear();
      updateText("");
      term.processExit();
    }

    if (name == "m") {
      // menu
      console.log("Not implemented");
    }

    if (name == " ") {
      // proceed
      i = Math.min(i + 1, lines.length - 1);
      if (!disableSend) updateText(lines[i]);
      drawLines(lines, i, disableSend);
    }

    if (name == "b") {
      // back
      i = Math.max(i - 1, 0);
      if (!disableSend) updateText(lines[i]);
      drawLines(lines, i, disableSend);
    }

    if (name == "c") {
      // clear screen
      updateText("");
      console.log("\nCleared screen");
    }

    if (name == "v") {
      // disable send (and clear)
      disableSend = !disableSend;
      if (disableSend) {
        updateText("");
        drawLines(lines, i, disableSend);
        console.log("\nDisabled send");
      } else {
        updateText(lines[i]);
        drawLines(lines, i, disableSend);
        console.log("\nEnabled send");
      }
    }

    if (name == "r") {
      // refresh file
      const percent = i / lines.length;
      lines = getLines(file);
      i = Math.floor(percent * lines.length);

      // disable send
      updateText("");
      disableSend = true;
      drawLines(lines, i, disableSend);
    }
  });
}

function drawLines(lines, index, disableSend) {
  lines = lines.map((line) => {
    if (line.startsWith("# ")) return `^Y${line}^`;
    if (line.startsWith("[") && line.endsWith("]")) return `^c${line}^`;
    return line;
  });
  console.log(lines);
  const height = term.height - 2;
  const middle = Math.floor(height / 2);
  const paddedLines = [
    ...Array.from({ length: middle - index }, () => [""]),
    ...lines.slice(Math.max(index - middle, 0), Math.max(index - 1, 0)).map((t) => [t]),
    [lines[Math.max(index - 1, 0)], " Previous"],
    [""],
    [lines[index], " Current"],
    [""],
    [lines[index + 1], " Next"],
    ...lines.slice(index + 2, index + middle).map((t) => [t]),
  ];
  term.clear();
  term.table(paddedLines, {
    hasBorder: false,
    contentHasMarkup: true,
    textAttr: { bgColor: "default", color: disableSend ? "red" : "default" },
    width: term.width,
    fit: true,
  });
}
