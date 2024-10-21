<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Globals

You can declare [globally available singletons](../slint/src/language/syntax/globals) in your
`.slint` files. If exported, these singletons are accessible as properties on your main
componen instance. Each global singleton is represented by an object with properties and callbacks,
similar to API that's created for your `.slint` component.

For example the following `.slint` markup defines a global `Logic` singleton that's also exported:

```
export global Logic {
    callback to_uppercase(string) -> string;
}
```

Assuming this global is used together with the `MyComponent` from the
previous section, you can access `Logic` like this:

```js
import * as slint from "slint-ui";

let ui = slint.loadFile(new URL("ui/my-component.slint", import.meta.url));
let component = new ui.MyComponent();

component.Logic.to_upper_case = (str) => {
    return str.toUpperCase();
};
```

**Note**: Global singletons are instantiated once per component. When declaring multiple components for `export` to JavaScript,
each instance will have their own instance of associated globals singletons.
