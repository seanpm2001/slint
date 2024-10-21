<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Setting and Invoking Callbacks

[Callbacks](../slint/src/language/syntax/callbacks) declared in `.slint` files are visible as JavaScript function properties on the component instance. Invoke them
as function to invoke the callback, and assign JavaScript functions to set the callback handler.

**`ui/my-component.slint`**

```
export component MyComponent inherits Window {
    callback clicked <=> i-touch-area.clicked;

    width: 400px;
    height: 200px;

    i-touch-area := TouchArea {}
}
```

**`main.mjs`**

```js
import * as slint from "slint-ui";

let ui = slint.loadFile(new URL("ui/my-component.slint", import.meta.url));
let component = new ui.MyComponent();

// connect to a callback
component.clicked = function() { console.log("hello"); };
// emit a callback
component.clicked();
```
