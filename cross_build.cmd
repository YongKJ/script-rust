
:: SET DOCKER_HOST=tcp://192.168.3.25:2375

:: cross build --target=aarch64-unknown-linux-musl

:: ===================================================================================================

:: cargo build --target=armv7-linux-androideabi

:: cargo build --target=arm-linux-androideabi

:: cargo build --target=aarch64-linux-android

:: cargo build --target=arm-unknown-linux-gnueabihf

:: cargo build --target=aarch64-unknown-linux-gnu

:: cargo build --target=aarch64-unknown-linux-musl

:: cargo build --target=x86_64-unknown-linux-gnu

:: cargo build --target=x86_64-unknown-linux-musl

:: cargo build --target=i686-unknown-linux-musl

cargo build --target=x86_64-pc-windows-msvc --release

:: cargo build --target=x86_64-pc-windows-gnu

:: cargo build --target=i686-pc-windows-msvc

:: cargo build --target=i686-pc-windows-gnu
