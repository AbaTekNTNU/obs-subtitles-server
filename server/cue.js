const term = require("terminal-kit").terminal;

module.exports = {
  start: function (updateText) {
    //const file = getFile();
    const text = getInput(updateText);
    updateText(text);
  },
};

//const getInput()
function getInput() {
  term.fullscreen(true);
  term.grabInput();
  term.on("key", (name, match, data) => {
    if (name == "q" || name == "CTRL_C") {
      term.clear();
      process.exit();
    }
    if (name == " ") {
    }
    console.log(name);
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
