# rock-paper-scissor
A simple rock paper scissor game in rust using AWS lambda. Users will be able to play against the computer. The user will be able to choose (1)rock, (2)paper, or (3)scissor, while the computer will randomly choose one of the three. 

### Usage
* `make format` to format code
* `make lint` to lint
* `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
* `make deploy` which is this`cargo lambda deploy`