
:: SET cc=D:\Software\scoop\apps\gcc\current\bin\gcc.exe

:: SET cxx=D:\Software\scoop\apps\gcc\current\bin\g++.exe

:: cargo build --target=aarch64-unknown-linux-musl

SET DOCKER_HOST=tcp://192.168.3.25:2375

cross build --target=aarch64-unknown-linux-musl

