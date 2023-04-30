# gitignore-generator

A simple way to generate `.gitignore` file using github api.

## Usage

```bash
❯ cargo install gitignore-generator

# binary is installed
❯ gitignore-generator
? Pick your language/template: › [Page 1/4]
❯ AL
  Actionscript
  Ada
  Agda
  Android
  AppEngine
  AppceleratorTitanium
  ArchLinuxPackages
  Autotools
  C
  C++
..
..

✔ Pick your language/template: · Rust
# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb
File already exists. Do you want to (a)ppend, (o)verwrite, or (i)gnore?
o
```

Check the generated ignorefile:
```bash
❯ cat gitignore.demo
# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb
```
