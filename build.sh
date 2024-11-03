if [ "$1" == "-r" ]; then
    cargo build --release
    cp target/release/interpreter ./bin/rel-interpreter
else
    cargo build 
    cp target/debug/interpreter ./bin/dbg-interpreter
fi


