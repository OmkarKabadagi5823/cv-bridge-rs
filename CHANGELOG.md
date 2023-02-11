# CHANGELOG.md

## 0.3.3
Changes:
  - bump the opencv version to 0.76.4 to possibly fix readonly issue causing doc build failure (check the issue here [opencv-rust #412](https://github.com/twistedfall/opencv-rust/issues/412))

## 0.3.2
Changes:
  - replace dependency on `rosrust_msg` crate with `rosrust`
  - create build script to generate messages
  - add examples directory
  - add more msgs
  
Bugfixes:
  - fix the persistent doc build failure
  
## 0.3.1

Bugfixes:
  - fix failed doc build by manual generation of ros messages

## 0.2.0 -> 0.3.0

Features:
  - remove dependency on `image` crate
  - added support for more encodings
  - added support for 16-bit depth channels
  - changes in public API
