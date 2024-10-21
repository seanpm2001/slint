<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Getting Started (Node.js)

1. In a new directory, create a new Node.js project by calling [`npm init`](https://docs.npmjs.com/cli/v10/commands/npm-init).
2. Install Slint for your project using [`npm install slint-ui`](https://docs.npmjs.com/cli/v10/commands/npm-install).
3. Create a new file called `main.slint` with the following contents:

```
import { AboutSlint, Button, VerticalBox } from "std-widgets.slint";
export component Demo inherits Window {
    in-out property <string> greeting <=> label.text;
    VerticalBox {
        alignment: start;
        label := Text {
            text: "Hello World!";
            font-size: 24px;
            horizontal-alignment: center;
        }
        AboutSlint {
            preferred-height: 150px;
        }
        HorizontalLayout { alignment: center; Button { text: "OK!"; } }
    }
}
```

This file declares the user interface.

4. Create a new file called `index.mjs` with the following contents:

```js
import * as slint from "slint-ui";
let ui = slint.loadFile(new URL("main.slint", import.meta.url));
let demo = new ui.Demo();

await demo.run();
```

This is your main JavaScript entry point:

* Import the Slint API as an [ECMAScript module](https://nodejs.org/api/esm.html#modules-ecmascript-modules) module. If you prefer you can
  also import it as [CommonJS](https://nodejs.org/api/modules.html#modules-commonjs-modules) module.
* Invoke `loadFile()` to compile and load the `.slint` file.
* Instantiate the `Demo` component declared in `main.slint`.
* Run it by showing it on the screen and reacting to user input.

5. Run the example with `node index.mjs`

For a complete example, see [/examples/todo/node](https://github.com/slint-ui/slint/tree/master/examples/todo/node).
