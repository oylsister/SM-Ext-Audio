# SM-Ext-Audio

Linux Building
```
// dependency
snap install rustup --classic
sudo apt-get install gcc-multilib libc6-dev-i386
sudo apt-get install g++-multilib gcc-multilib
sudo apt-get install gcc-i686-linux-gnu g++-i686-linux-gnu

git clone https://github.com/alliedmodders/sourcemod --recursive -b 1.10-dev deps/sourcemod
git clone https://github.com/alliedmodders/metamod-source -b 1.10-dev deps/mmsource
git clone https://github.com/alliedmodders/hl2sdk -b csgo deps/hl2sdk-csgo

export SOURCEMOD="/root/deps/sourcemod"
export MMSOURCE="/root/deps/mmsource"
export HL2SDKCSGO="/root/deps/hl2sdk-csgo"
rustup target add i686-unknown-linux-gn
cargo build --target i686-unknown-linux-gnu --release
```