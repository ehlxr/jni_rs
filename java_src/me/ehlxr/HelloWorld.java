package me.ehlxr;

public class HelloWorld {
    static {
        // Linux: export LD_LIBRARY_PATH=/Users/ehlxr/Desktop/jni_rs/mylib/target/debug
        // Mac: export JAVA_LIBRARY_PATH=/Users/ehlxr/Desktop/jni_rs/mylib/target/debug
        System.loadLibrary("mylib");
    }
    private static native String hello(String input);

    public static void main(String[] args) {
        String output = HelloWorld.hello("Java");
        System.out.println(output);
    }
}