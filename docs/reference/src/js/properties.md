<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

### Accessing a Properties

[Properties](../slint/src/language/syntax/properties) declared as `out` or `in-out` in `.slint` files are visible as JavaScript properties on the component instance.

**`main.slint`**
export component MainWindow {
    in-out property <string> name;
    in-out property <int> age: 42;
}

```js
let ui = slint.loadFile(new URL("main.slint", import.meta.url));
let instance = new ui.MainWindow();
console.log(instance.age); // Prints 42
instance.name = "Joe";
```
