<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Getting Started (Deno)

1. Create a new file called `main.slint` with the following contents:

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

2. Create a new file called `deno.json` (a [Deno Import Map](https://docs.deno.com/runtime/manual/basics/import_maps))
   with the following contents:

```json
{
  "imports": {
    "slint-ui": "npm:slint-ui"
  }
}
```

3. Create a new file called `index.ts` with the following contents:

```ts
import * as slint from "slint-ui";
let ui = slint.loadFile(new URL("main.slint", import.meta.url));
let demo = new ui.Demo();

await demo.run();
```

This is your main JavaScript entry point:

* Import the Slint API as an [ECMAScript module](https://nodejs.org/api/esm.html#modules-ecmascript-modules) module through Deno's
  NPM compatibility layer.
* Invoke `loadFile()` to compile and load the `.slint` file.
* Instantiate the `Demo` component declared in `main.slint`.
* Run it by showing it on the screen and reacting to user input.

1. Run the example with `deno run --allow-read --allow-ffi --allow-sys index.ts`
