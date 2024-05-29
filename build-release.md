

## BUILD FOR RELEASE 

windows 

rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64

cargo build --release --target x86_64-pc-windows-gnu
