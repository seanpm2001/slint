<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Getting Started (bun)

1. In a new directory, create a new `bun` project by calling [`bun init`](https://bun.sh/docs/cli/init).
2. Install Slint for your project using [`bun install slint-ui`](https://bun.sh/docs/cli/install).
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

4. Clear the conent of`index.ts` and add the following code:

```ts
import * as slint from "slint-ui";
let ui = slint.loadFile(new URL("main.slint", import.meta.url)) as any;
let demo = new ui.Demo();

await demo.run();
```

This is your main TypeScript entry point:

* Import the Slint API as an [ECMAScript module](https://nodejs.org/api/esm.html#modules-ecmascript-modules) module. 
* Invoke `loadFile()` to compile and load the `.slint` file.
* Instantiate the `Demo` component declared in `main.slint`.
* Run it by showing it on the screen and reacting to user input.

5. Run the example with `bun run index.ts`
