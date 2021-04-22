#!/bin/bash
set -e

pushd ..
./gradlew assemble
sudo cp build/libs/libopenlimits_java.so /usr/lib/jni
popd
javac -cp .:../build/libs/openlimits-java-0.1.4.jar Example.java
java -cp .:../build/libs/openlimits-java-0.1.4.jar Example

