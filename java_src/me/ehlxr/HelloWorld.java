package me.ehlxr;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.util.HashMap;

class HelloWorld {
    static {
        // Linux: export LD_LIBRARY_PATH=/Users/ehlxr/Workspaces/Rust/jni_rs/mylib/target/debug
        // Mac: export JAVA_LIBRARY_PATH=/Users/ehlxr/Workspaces/Rust/jni_rs/mylib/target/debug
        System.loadLibrary("mylib");
    }

    public Long no;
    private String name;
    public int age;
    public List<String> ls;
    public Map<String, Long> map;

    private static native String hello(String input);

    private static native byte[] helloByte(byte[] input);

    private static native void factAndCallMeBack(int n, HelloWorld callback);

    private static native long counterNew(HelloWorld callback);

    private static native void counterIncrement(long counter_ptr);

    private static native void counterDestroy(long counter_ptr);

    private static native void asyncComputation(HelloWorld callback);

    private static native String getFiled(HelloWorld param);

    public static void main(String[] args) {
        String output = HelloWorld.hello("Java");
        System.out.println(output);

        // byte[] outputByte = HelloWorld.helloByte("byte".getBytes());
        // System.out.println(outputByte);

        // HelloWorld.factAndCallMeBack(6, new HelloWorld());

        // long counter_ptr = counterNew(new HelloWorld());

        // for (int i = 0; i < 5; i++) {
        //     counterIncrement(counter_ptr);
        // }

        // counterDestroy(counter_ptr);

        // System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().getId() + ")");
        // asyncComputation(new HelloWorld());

        List<String> ls = new ArrayList<>();
        ls.add("ls1");
        ls.add("ls2");
        ls.add("ls3");

        Map<String, Long> map = new HashMap<>();
        map.put("k1", 1L);
        map.put("k2", 2L);
        map.put("k3", 3L);

        HelloWorld hw = new HelloWorld();
        hw.setName("Jack");
        hw.no = 123434555L;
        hw.age = 30;
        hw.ls = ls;
        hw.map = map;
        System.out.println(HelloWorld.getFiled(hw));
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void factCallback(int res) {
        System.out.println("factCallback: res = " + res);
    }

    public void counterCallback(int count) {
        System.out.println("counterCallback: count = " + count);
    }

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
    }
}
