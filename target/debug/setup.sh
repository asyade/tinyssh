NDK_TOOLCHAIN_ARM=$NDK_HOME/toolchains/arm-linux-androideabi-4.9/prebuilt/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)/
NDK_TOOLCHAIN_ARM64=$NDK_HOME/toolchains/aarch64-linux-android-4.9/prebuilt/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)/
NDK_SYSROOT=$NDK_HOME/platforms/android-21
cat > .cargo/config <<END
[target.arm-linux-androideabi]
linker = "$NDK_TOOLCHAIN_ARM/bin/arm-linux-androideabi-gcc"
ar = "$NDK_TOOLCHAIN_ARM/bin/arm-linux-androideabi-ar"
rustflags = ["-Clink-args=--sysroot=$NDK_SYSROOT/arch-arm"]

[target.aarch64-linux-android]
linker = "$NDK_TOOLCHAIN_ARM64/bin/aarch64-linux-android-gcc"
ar = "$NDK_TOOLCHAIN_ARM64/bin/aarch64-linux-android-ar"
rustflags = ["-Clink-args=--sysroot=$NDK_SYSROOT/arch-arm64"]
END
