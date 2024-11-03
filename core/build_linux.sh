cargo build --release
rm -rf ../lib/src/lib/libnewtonium.so
cp target/release/libnewtonium.so ../lib/src/lib/libnewtonium.so