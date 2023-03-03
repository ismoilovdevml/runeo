#!/bin/bash


curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


git clone https://github.com/ismoilovdevml/runeo.git

cd runeo

cargo run

