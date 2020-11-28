#!/bin/bash

BASE_DIR=$PWD
SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
ROOT_DIR="$SCRIPT_DIR"

if [[ -z "${ARTIFACT_VERSION}" ]]; then
  ARTIFACT_VERSION="$(cat "CLIENT_ARTIFACT_VERSION")"
fi

PLATFORM="$(uname -s)"

# File extensions of native libraries are platform-specific
case "$PLATFORM" in
  Linux*) FILE_EXT="so";;
  Darwin*) FILE_EXT="dylib";;
  CYGWIN*) FILE_EXT="dll";;
  MINGW*) FILE_EXT="dll";;
  *) echo >&2 "Unknown/Unsupported platform: $PLATFORM" && exit 1;;
esac

SOURCE_PATH="$ROOT_DIR/build/libs/"

cd "$SOURCE_PATH" || exit;
mkdir libs;
cp "libopenlimits_java.$FILE_EXT" libs;
jar xvf "openlimits-java-$ARTIFACT_VERSION.jar";
rm -rf "openlimits-java-$ARTIFACT_VERSION.jar";
jar cf "nash-jni-$ARTIFACT_VERSION.jar" *;
echo "CREATED ${SOURCE_PATH}nash-jni-${ARTIFACT_VERSION}.jar";
rm -rf libs;
cd "$BASE_DIR" || exit;
