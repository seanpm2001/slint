<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0 -->

# Setting and Invoking Callbacks

[Callbacks](src/language/syntax/callbacks) declared in `.slint` files are visible as callable properties on the component instance. Invoke them
as function to invoke the callback, and assign Python callables to set the callback handler.

Callbacks in Slint can be defined using the `callback` keyword and can be connected to a callback of an other component
using the `<=>` syntax.

**`my-component.slint`**

```slint
export component MyComponent inherits Window {
    callback clicked <=> i-touch-area.clicked;

    width: 400px;
    height: 200px;

    i-touch-area := TouchArea {}
}
```

The callbacks in Slint are exposed as properties and that can be called as a function.

**`main.py`**

```python
import slint

component = slint.loader.my_component.MyComponent()
# connect to a callback

def clicked():
    print("hello")

component.clicked = clicked
// invoke a callback
component.clicked();
```

Another way to set callbacks is to sub-class and use the `@slint.callback` decorator:

```python
import slint

class Component(slint.loader.my_component.MyComponent):
    @slint.callback
    def clicked(self):
        print("hello")

component = Component()
```

The `@slint.callback()` decorator accepts a `name` named argument, when the name of the method
does not match the name of the callback in the `.slint` file. Similarly, a `global_name` argument
can be used to bind a method to a callback in a global singleton.
``
