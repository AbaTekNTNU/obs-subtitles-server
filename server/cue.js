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
  runSelector(lines, updateText);
}

async function getFile(subs_dir) {
  term.cyan("Choose a subtitle:\n");
  const items = readdirSync("subs");
  const file = await term.gridMenu(items).promise;
  return path.join(subs_dir, file.selectedText);
}

function getLines(file) {
  const lines1 = Array.from({ length: 100 }, (_, i) => `Linje ${i + 1}`);
  const lines2 = lorem.split(" ");
  const lines2c = Array.from({ length: Math.ceil(lines2.length / 5) }, (_, i) =>
    lines2.slice(i * 5, i * 5 + 5).join(" ")
  );
  const content = readFileSync(file).toString().replaceAll("\r", "").split("\n");
  const lines = ["", ...content, ""];
  return lines;
}

async function runSelector(lines, updateText) {
  let i = 0;
  let disableSend = false;
  drawLines(lines, i);

  term.on("key", (name, match, data) => {
    if (name == "q" || name == "CTRL_C") {
      // quit
      //term.clear();
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
      console.log("Not implemented");
    }
  });
}

function drawLines(lines, index, disableSend) {
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
    textAttr: { bgColor: "default", color: disableSend ? "red" : "default" },
    width: term.width,
    fit: true,
  });
}

/* Pseudocode:
lines = readsubs
linenumber = 0
log(nextlines)
disableSend = false
loop key {
    key = q {
        return ok
    }
    key = space {
        if linenumber < len {
            line = lines[linenumber]
            linenumber++
            if !disableSend {
                updateText(line)
            }
            log(nextlines)
        } else {
            updateText("")
            log("No more lines to send")
        }
    }
    key = m {
        log("Going back to menu")
        updateText("")
        reset()
    }
    key = b {
        if linenumber > 1 {
            linenumber--
            line = lines[linenumber]
            if !disableSend {
                updateText(line)
            }
            log(nextlines)
        }
    }
    key = c {
        updateText("")
        log("Cleared screen")
    }
    key = v {
        disableSend = !disableSend;
        if disableSend {
            //updateText("")
            log("Disabled sending")
        } else {
            //updateText(lines[linenumber - 1])
            log("Enabled sending")
        }
        //log(nextlines)
    }
}
*/

/* UI:         Col 2
Linje 1
Linje 2
Linje 3  ...   Previous

Linje 4  ...   Current

Linje 5  ...   Next
Linje 6
Linje 7
(end at terminal height)
*/

const lorem =
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ac massa ultricies, pretium lorem nec, feugiat nulla. Proin facilisis at metus ut dignissim. Etiam eu nibh tellus. Nullam faucibus accumsan nibh, sit amet vulputate mauris convallis nec. Donec et ex vel nibh convallis consectetur vitae nec sem. Nunc nec lacus nulla. Nam rhoncus ornare gravida. Cras laoreet neque lorem, ut posuere metus ullamcorper vitae. Nulla maximus mattis nunc, sit amet dapibus magna ultricies et. Phasellus tortor metus, aliquet sodales nunc sed, fermentum eleifend quam. Vivamus mattis, augue nec dapibus viverra, ante nunc imperdiet libero, et venenatis felis velit nec quam. Donec nisi odio, ornare non libero non, vulputate vestibulum augue. Nam ut lacinia nisl. Quisque convallis, orci ut aliquet fermentum, ante mi accumsan leo, eget scelerisque ipsum nisi eu lorem. Nulla non blandit neque, sit amet dictum elit. Ut scelerisque dui id nulla iaculis, in tempor elit posuere. Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse finibus rutrum hendrerit. Aenean ante lectus, posuere in lorem quis, eleifend sagittis quam. In hac habitasse platea dictumst. Nunc sed risus in eros imperdiet tincidunt. Ut tristique ullamcorper libero, at porttitor odio elementum gravida. Duis non purus sed dolor euismod convallis eu eu metus. Nam at erat leo. Praesent tristique, velit sed egestas mollis, tellus risus rutrum ex, vitae varius augue est ut tellus. Integer sodales mattis diam, et ullamcorper sapien efficitur quis. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Sed mattis nisi et eros tempus suscipit. Vestibulum accumsan vehicula libero nec cursus. Proin efficitur lobortis accumsan. Cras mattis nulla vel justo mollis dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent elementum nibh a augue consectetur ultricies. Ut venenatis in lorem sit amet ornare. Ut ultrices mi sed augue gravida, a mollis neque laoreet. Aenean ligula dui, fermentum consequat justo eget, iaculis pellentesque neque. Etiam commodo augue eget iaculis rhoncus. Phasellus id sem arcu. Donec non rutrum mi. Suspendisse non ornare risus. Phasellus viverra euismod sodales. Donec luctus porta odio, eget placerat dui semper at. Sed nec justo a nibh sodales sagittis eu ac lorem. Nulla facilisi. Nunc faucibus mattis iaculis. Aliquam gravida ut leo id posuere. Integer sagittis augue at lectus laoreet sagittis. Duis luctus nibh a leo venenatis, nec consequat diam scelerisque. Fusce elit ante, efficitur eu tellus eu, convallis mattis nibh. Nunc tellus dui, cursus eu vulputate in, porta eu dui. Aliquam egestas viverra risus, ac luctus erat pulvinar ut. Maecenas velit ante, sagittis eget tincidunt eu, eleifend vel sapien. Etiam augue risus, ornare ac justo id, cursus ullamcorper est. Morbi sit amet purus nec eros vehicula maximus. Duis sodales euismod lobortis. Donec vulputate bibendum tellus, a blandit urna hendrerit at. Vestibulum a mollis augue. Vestibulum consequat dui a orci sollicitudin lobortis. Donec iaculis ac arcu condimentum blandit. Morbi tempor est sit amet turpis feugiat, nec placerat sapien imperdiet. Donec ut risus eget leo consequat bibendum eget in nisi.";
