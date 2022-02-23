java_run: lib
	cd java_src	&& \
	javac me/ehlxr/HelloWorld.java && \
	java -Djava.library.path=../mylib/target/debug/ me.ehlxr.HelloWorld
.PHONY: lib

javah:
	cd java_src	&& \
	javah me.ehlxr.HelloWorld

lib:
	cd mylib && cargo build