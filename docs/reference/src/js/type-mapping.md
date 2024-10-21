<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Type Mappings

The types used for properties in .slint design markup each translate to specific types in JavaScript. The follow table summarizes the entire mapping:

| `.slint` Type | JavaScript Type | Notes |
| --- | --- | --- |
| `int` | `Number` | |
| `bool` | `Boolean` | |
| `float` | `Number` | |
| `string` | `String` | |
| `color` | {@link RgbaColor} | |
| `brush` | {@link Brush} | |
| `image` | {@link ImageData} | |
| `length` | `Number` | |
| `physical_length` | `Number` | |
| `duration` | `Number` | The number of milliseconds |
| `angle` | `Number` | The angle in degrees |
| `relative-font-size` | `Number` | Relative font size factor that is multiplied with the `Window.default-font-size` and can be converted to a `length`. |
| structure | `Object` | Structures are mapped to JavaScript objects where each structure field is a property. |
| array | {@link Model} | |

## Arrays and Models

[Array properties](../slint/src/language/syntax/types#arrays-and-models) can be set from JavaScript by passing
either `Array` objects or implementations of the {@link Model} interface.

When passing a JavaScript `Array` object, the contents of the array are copied. Any changes to the JavaScript afterwards will not be visible on the Slint side.

Reading a Slint array property from JavaScript will always return a @{link Model}.

```js
component.model = [1, 2, 3];
// component.model.push(4); // does not work, because assignment creates a copy.
// Use re-assignment instead.
component.model = component.model.concat(4);
```

Another option is to set an object that implements the {@link Model} interface.

## structs

An exported struct can be created either by defing of an object literal or by using the new keyword.

**`my-component.slint`**

```
export struct Person {
    name: string,
    age: int
}

export component MyComponent inherits Window {
    in-out property <Person> person;
}
```

**`main.js`**

```js

import * as slint from "slint-ui";

let ui = slint.loadFile(new URL("my-component.slint", import.meta.url));
let component = new ui.MyComponent();

// object literal
component.person = { name: "Peter", age: 22 };

// new keyword (sets property values to default e.g. '' for string)
component.person = new ui.Person();

// new keyword with parameters
component.person = new ui.Person({ name: "Tim", age: 30 });
```

## enums

A value of an exported enum can be set as string or by usign the value from the exported enum.

**`my-component.slint`**

```
export enum Position {
    top,
    bottom
}

export component MyComponent inherits Window {
    in-out property <Position> position;
}
```

**`main.js`**

```js

import * as slint from "slint-ui";

let ui = slint.loadFile(new URL("my-component.slint", import.meta.url));
let component = new ui.MyComponent();

// set enum value as string
component.position = "top";

// use the value of the enum
component.position = ui.Position.bottom;
```
