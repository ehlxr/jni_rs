Rust build

```shell
cd mylib
cargo build
```

Java build

```shell
cd java_src
javac me/ehlxr/HelloWorld.java
```

Java run

```shell
cd java_src

# Linux: export LD_LIBRARY_PATH=../mylib/target/debug/
# export JAVA_LIBRARY_PATH=../mylib/target/debug/

java -Djava.library.path=../mylib/target/debug/ me.ehlxr.HelloWorld
```