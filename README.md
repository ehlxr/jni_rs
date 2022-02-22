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

Run Java
```shell
cd java_src

// Linux: export LD_LIBRARY_PATH=../mylib/target/debug/
export JAVA_LIBRARY_PATH=../mylib/target/debug/

java me.ehlxr.HelloWorld
```