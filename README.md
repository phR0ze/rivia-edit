# rivia-edit
[![license-badge](https://img.shields.io/crates/l/rivia-edit.svg)](https://opensource.org/licenses/MIT)

***Orchestration for editing files in a repeatable way***

### Quick links
* [Usage](#usage)
  * [Rustc requirments](#rustc-requirements)
* [Contribute](#contribute)
  * [Dev Environment](#dev-environment)
    * [Automatic version](#automatic-version)
  * [Testing](#testing)
    * [Test in container](#test-in-container)
* [License](#license)
  * [Contribution](#contribution)
* [Backlog](#backlog)
* [Changelog](#changelog)

# Usage <a name="usage"/></a>

### Rustc requirements
This minimum rustc requirement is driven by the enhancements made to [Rust's `std::error::Error`
handling improvements](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)

# Contribute
Pull requests are always welcome. However understand that they will be evaluated purely on whether
or not the change fits with my goals/ideals for the project.

**Project guidelines**:
* ***Chaining*** - ensure Rust's functional chaining style isn't impeded by additions
* ***Brevity*** - keep the naming as concise as possible while not infringing on clarity
* ***Clarity*** - keep the naming as unambiguous as possible while not infringing on brevity
* ***Performance*** - keep convenience functions as performant as possible while calling out significant costs
* ***Speed*** - provide ergonomic functions similar to rapid development languages
* ***Comfort*** - use naming and concepts in similar ways to popular languages

## Dev Environment

### Automatic version
Enable the git hooks to have the version automatically incremented on commits

```bash
cd ~/Projects/rivia-edit
git config core.hooksPath .githooks
```

## Testing

### Test in container
TBD

# License
This project is licensed under either of:
 * MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
 * Apache License, Version 2.0 [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, shall be dual licensed as above, without any additional terms or conditions.

---

# Backlog

# Changelog
