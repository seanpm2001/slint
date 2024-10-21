<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0 -->

# Accessing Properties

[Properties](../slint/src/language/syntax/properties) declared as `out` or `in-out` in `.slint` files are visible as  properties on the component instance.

```python
main_window.counter = 42
print(main_window.counter)
```
``
