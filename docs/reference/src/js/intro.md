<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->

# Introduction

[![npm](https://img.shields.io/npm/v/slint-ui)](https://www.npmjs.com/package/slint-ui)

[Slint](https://slint.dev/) is a UI toolkit that supports different programming languages.
Slint-node is the integration with [Node.js](https://nodejs.org/en), [Deno](https://deno.com),
and [bun](https://bun.sh).

To get started you use the [walk-through tutorial](https://slint.dev/docs/slint/src/quickstart).
We also have a [Getting Started Template](https://github.com/slint-ui/slint-nodejs-template) repository with
the code of a minimal application using Slint that can be used as a starting point to your program.

**Warning: Beta**
Slint for Javascript and Typescript is still in beta stage of development: APIs
might change and important features are still being developed.

## Prerequisites

To use Slint with Node.js, ensure the following programs are installed:

  * **[Node.js](https://nodejs.org/download/release/)** (v16. or newer)
  * **[npm](https://www.npmjs.com/)**

To use Slint with Deno, ensure the following programs are installed:

  * **[Deno](https://docs.deno.com/runtime/manual)**

### Building from Source

Slint-node comes with pre-built binaries for macOS, Linux, and Windows. If you'd like to use Slint-node on a system
without pre-built binaries, you need to additional software:

  * **[Rust compiler](https://www.rust-lang.org/tools/install)** (1.77 or newer)
 * Depending on your operating system, you may need additional components. For a list of required system libraries,
    see <https://github.com/slint-ui/slint/blob/master/docs/building.md#prerequisites>.
