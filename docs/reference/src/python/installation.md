<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0 -->

# Installation

## Prerequisites

 * [Python 3](https://python.org/)
 * [pip](https://pypi.org/project/pip/)
 * [Pipenv](https://pipenv.pypa.io/en/latest/installation.html#installing-pipenv)

## Installation

Slint can be installed with `pip` from the [Python Package Index](https://pypi.org):

```
pip install slint
```

The installation will use binaries provided vi macOS, Windows, and Linux for various architectures. If your target platform is not covered by binaries,
`pip` will automatically build Slint from source. If that happens, you need common software development tools on your machine, as well as [Rust](https://www.rust-lang.org/learn/get-started).

## Try it out

If you want to just play with this, you can try running our Python port of the [printer demo](../../examples/printerdemo/python/README.md):

```bash
cd examples/printerdemo/python
pipenv update
pipenv run python main.py
```

## Quick Start

1. Add Slint Python Package Index to your Python project: `pipenv install slint`
2. Create a file called `app-window.slint`:

```slint
import { Button, VerticalBox } from "std-widgets.slint";

export component AppWindow inherits Window {
    in-out property<int> counter: 42;
    callback request-increase-value();
    VerticalBox {
        Text {
            text: "Counter: \{root.counter}";
        }
        Button {
            text: "Increase value";
            clicked => {
                root.request-increase-value();
            }
        }
    }
}
```

1. Create a file called `main.py`:

```python
import slint

# slint.loader will look in `sys.path` for `app-window.slint`.
class App(slint.loader.app_window.AppWindow):
    @slint.callback
    def request_increase_value(self):
        self.counter = self.counter + 1

app = App()
app.run()
```

4. Run it with `pipenv run python main.py`
``
