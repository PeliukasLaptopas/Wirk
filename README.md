I have downloaded SDL2 (SDL2-devel-2.0.14-VC.zip) from https://www.libsdl.org/download-2.0.php and added /x64/* files to C:\Users\*\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ in order for SDL to work

Machine needs visual c++ installed. Just simply download visual studio 2019 or you will get this error while compiling:
"please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option"
Just download vs from here:
https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16
and install c++ option

Box2D requiriments
if cmake is not installed you will get this error while compiling:
![image](https://user-images.githubusercontent.com/38985678/112358177-c328fd00-8cd8-11eb-9e83-629307c38f25.png)

download and install cmake-3.20.0-windows-x86_64.msi
![image](https://user-images.githubusercontent.com/38985678/112358303-e489e900-8cd8-11eb-9814-ec333b93d74c.png)


This is a project that uses my engine
![image](https://user-images.githubusercontent.com/38985678/111040399-7a04bd80-843b-11eb-9734-81dddbca835a.png)

![image](https://user-images.githubusercontent.com/38985678/113760946-8a007c00-971f-11eb-8b19-a2eba59a1b96.png)
![image](https://user-images.githubusercontent.com/38985678/111040417-8b4dca00-843b-11eb-895e-96c86b3a62dc.png)

Supports
Vsync
fps calculator
fps limiter
