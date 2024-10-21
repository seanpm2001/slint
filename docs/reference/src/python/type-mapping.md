<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0 -->

# Type Mappings

The types used for properties in the Slint Language each translate to specific types in Python. The follow table summarizes the entire mapping:

| `.slint` Type | Python Type | Notes |
| --- | --- | --- |
| `int` | `int` | |
| `float` | `float` | |
| `string` | `str` | |
| `color` | `slint.Color` |  |
| `brush` | `slint.Brush` |  |
| `image` | `slint.Image` |  |
| `length` | `float` | |
| `physical_length` | `float` | |
| `duration` | `float` | The number of milliseconds |
| `angle` | `float` | The angle in degrees |
| structure | `dict`/`Struct` | When reading, structures are mapped to data classes, when writing dicts are also accepted. |
| array | `slint.Model` | |

### Arrays and Models

[Array properties](../slint/src/language/syntax/types#arrays-and-models) can be set from Python by passing
subclasses of `slint.Model`.

Use the `slint.ListModel` class to construct a model from an iterable.

```js
component.model = slint.ListModel([1, 2, 3]);
component.model.append(4)
del component.model[0]
```

When sub-classing `slint.Model`, provide the following methods:

```python
    def row_count(self):
        """Return the number of rows in your model"""

    def row_data(self, row):
        """Return data at specified row"""

    def set_row_data(self, row, data):
        """For read-write models, store data in the given row. When done call set.notify_row_changed:"
        ..."""
        self.notify_row_changed(row)
```

When adding/inserting rows, call `notify_row_added(row, count)` on the super class. Similarly, removal
requires notifying Slint by calling `notify_row_removed(row, count)`.

### Structs

Structs declared in Slint and exposed to Python via `export` are accessible in the namespace returned
when [instantiating a component](#instantiating-a-component).

**`app.slint`**

```slint
export struct MyData {
    name: string,
    age: int
}

export component MainWindow inherits Window {
    in-out property <MyData> data;
}
```

**`main.py`**

The exported `MyData` struct can be constructed

```python
import slint
# Look for for `app.slint` in `sys.path`:
main_window = slint.loader.app.MainWindow()

data = slint.loader.app.MyData(name = "Simon")
data.age = 10
main_window.data = data
```
