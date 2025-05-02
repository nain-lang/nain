<div align="center">
	<h1>Nain</h1>
	<img src="media/nain-logo.png" alt="Nain Logo" width="100" height="100" />
</div>

<p align="center">
  <a href="#introduction">Introduction</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#acknowledge">Acknowledge</a> •
  <a href="#license">License</a>
</p>

## Introduction

Nain is a **free & open-source**, **safe**, **fast** and **secure** systems programming language.

## Features

- **⚡ Fast:** Nain is **really fast** thanks to its **lazily-evaluated, JIT compiler**.
- **🔒 Secure:** Nain will catch all **memory-related issues at compile-time**.
- **🖥️ Embeddable:** Nain has a **really small runtime** making it **ideal** for **embedded systems**.
- **📦 Extensible:** In Nain, you can **extend** the language with anything you want, from **new paradigms** to **a whole language inside of Nain**.
---

## Installation

  <details>
    <summary>🪟 Windows</summary>

  To install Nain on Windows, run the following PowerShell script:

  ```powershell
  # Invoke-WebRequest -Uri https://raw.githubusercontent.com/nain-lang/nain/refs/heads/master/install.ps1 -OutFile install.ps1; .\install.ps1
  ```

  Note that the `#` is not part of the command.

  It will ask you **one question**: \[?\] What version of Nain do you want to install (**\[g\]it or \[r\]elease**)?
  - If you want to install the absolute latest version (**not recommended**), press <kbd>g</kbd>. Else, press <kbd>r</kbd>.

  </details>

  <details>
    <summary>🐧 Linux / 🍎 MacOS / 👹 FreeBSD</summary>

  To install Nain on Linux, MacOS, or FreeBSD, run the following command:

  ```bash
  $ curl -sSL https://raw.githubusercontent.com/nain-lang/nain/refs/heads/master/install.sh | bash
  ```

  Note that the `$` is not part of the command

  > !NOTE
  > Make sure you have `curl` installed on your system.

  </details>

### Test the installation

To test the installation, run:

```bash
$ nain -v
Nain version 0.1
© 2025 The Nain Language Contributors. Some rights reserved.
```

Again, note that **the `$` is not part of the command**

## Configuration

Most of Nain's configuration if project-specific and uses a `package.toml` file.

Here is a table with the various options:

-----------------------------------------------------------------------------------------------------------------------------------------------------------
| Option     | Description                                         | Default         | Type       | On Section  | Optional                                |
|------------|-----------------------------------------------------|-----------------|------------|-------------|-----------------------------------------|
| `name`     | The projet's name.                                  | `""`            | `string`   | `[project]` | No                                      |
| `version`  | The project's version (semver or whatever you use). | `""`            | `string`   | `[project]` | No                                      |
| `author`   | The project author/s.                               | `[""]`          | `[string]` | `[project]` | Yes (shown as *Anonymous*)              |
| `license`  | The [SPDX](https://spdx.org/) license identifier.   | `"MIT"`         | `string`   | `[project]` | No                                      |
| `repo`     | The project's repository-                           | `""`            | `string`   | `[project]` | Yes                                     |
| `docs`     | The project's documentation page.                   | `""`            | `string`   | `[project]` | Yes                                     |
| `homepage` | The project's web page.                             | `""`            | `string`   | `[project]` | Yes                                     |
| `exec`     | The executable to build.                            | `name`          | `string`   | `[[bin]]`   | Yes (defaults to the value of `name`)   |
| `source`   | The main source file                                | `"src/main.na"` | `string`   | `[[bin]]`   | Yes                                     |
| `<string>` | A dependency.                                       | ``              | `[version]`| `[deps]`    | Yes                                     |
-----------------------------------------------------------------------------------------------------------------------------------------------------------

There are more options documented on the [wiki](https://wiki.nainlang.xyz/wiki/Package.toml).

## Documentation

There are **tons** of docuemtations on the **[wiki](https://wiki.nainlang.xyz/)**

It also includes information about most popular libraries. All the content there is under
the Creative Commons Attribution-ShareAlike 4.0 International (CC-BY-SA 4.0).
So you can use it for whatever you want as long as you credit the contributors and
don't change the license.

## Acknowledgement

Thanks to these amazing people for making Nain possible:

- [**The Nain Language Contributors**](https://github.com/nain-lang/nain/graphs/contributors)
- [Maciej Hirsz](https://github.com/maciejhirsz) for the **[*logos*](https://github.com/maciejhirsz/logos) library** for the lexer.
- The volunteer editors of the [**Nain Wiki**](https://wiki.nainlang.xyz/)
- Everyone on the community for being nice and helping others.
- And **you** for using Nain!

## License

Nain is licensed under the [**GNU General Public License v3.0**](https://www.gnu.org/licenses/gpl-3.0.en.html) or the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
Whatever you like :)

---

<sub>*© 2025 The Nain Language Contributors. Some rights reserved.*</sub>
