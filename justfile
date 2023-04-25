default:
    just -l

run:
    cargo run
    cd testing && make clean && make && ./main.x
