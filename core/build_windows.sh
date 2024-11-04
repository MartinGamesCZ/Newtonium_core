docker start -ai newcore-build
cp -r package/* ../lib/src/lib
mv ../lib/src/lib/newtonium.dll ../lib/src/lib/libnewtonium.dll
sudo rm -rf package