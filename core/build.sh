cargo build --release
cargo xwin build --release --target=x86_64-pc-windows-msvc
rm -rf ../lib/src/lib/libnewtonium.so
rm -rf ../lib/src/lib/libnewtonium.dll
cp target/debug/libnewtonium.so ../lib/src/lib/libnewtonium.so
cp target/x86_64-pc-windows-msvc/release/newtonium.dll ../lib/src/lib/libnewtonium.dll