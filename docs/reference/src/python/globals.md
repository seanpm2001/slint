<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0 -->

### Accessing Globals

[Global Singletons](https://slint.dev/docs/slint/src/language/syntax/globals#global-singletons) are accessible in
Python as properties in the component instance:

```slint,ignore
export global PrinterJobQueue {
    in-out property <int> job-count;
}
```

```python
print("job count:", instance.PrinterJobQueue.job_count)
```

**Note**: Global singletons are instantiated once per component. When declaring multiple components for `export` to Python,
each instance will have their own instance of associated globals singletons.
``
