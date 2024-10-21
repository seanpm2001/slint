<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Instantiating a Component

Use the {@link loadFile} function to load a `.slint` file. Instantiate the [exported component](../slint/src/language/concepts/file)
with the new operator. Access exported callbacks and properties as JavaScript properties on the instantiated component. In addition,
the returned object implements the {@link ComponentHandle} interface, to show/hide the instance or access the window.

The following example shows how to instantiating a Slint component from JavaScript.

**`ui/main.slint`**

```
export component MainWindow inherits Window {
    callback clicked <=> i-touch-area.clicked;

    in property <int> counter;

    width: 400px;
    height: 200px;

    i-touch-area := TouchArea {}
}
```

The exported component is exposed as a type constructor. The type constructor takes as parameter
an object which allow to initialize the value of public properties or callbacks.

**`main.mjs`**

```js
import * as slint from "slint-ui";
// In this example, the main.slint file exports a module which
// has a counter property and a clicked callback
let ui = slint.loadFile(new URL("ui/main.slint", import.meta.url));
let component = new ui.MainWindow({
    counter: 42,
    clicked: function() { console.log("hello"); }
});
```
