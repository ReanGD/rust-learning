# rust-software-render

[master](https://github.com/ReanGD/rust-software-render/tree/master) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=master)](https://travis-ci.org/ReanGD/rust-software-render) [![Build status](https://ci.appveyor.com/api/projects/status/y32wy5vu51q9hscm/branch/master?svg=true)](https://ci.appveyor.com/project/ReanGD/rust-software-render/branch/master)

Подготовка окружения
--
Для сборки проекта у вас должны стоять:
- компилятор [Rust](http://www.rust-lang.org/) (для Windows тестируется только stable версия, для Linux: nightly, beta, stable)
- менеджер пакетов для rust - [Cargo](https://crates.io/)

Для Windows необходимо добавить папку с rustc и cargo в PATH

Дополнительные пакеты
--
Требуется установить библиотеку [sdl2](https://www.libsdl.org/download-2.0.php) и [sdl2_image](https://www.libsdl.org/projects/SDL_image/).

Для Arch-Linux ставится вот так:
```Shell
sudo pacman -S sdl2 sdl2_image
```
Для Ubuntu вот так:
```Shell
sudo add-apt-repository ppa:zoogie/sdl2-snapshots -y
sudo apt-get update -q
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

Для Windows:
- качаем [SDL2-devel-2.0.3-mingw.tar.gz](https://www.libsdl.org/release/SDL2-devel-2.0.3-mingw.tar.gz), распаковываем, ищем файл SDL2-2.0.3\TARGET-w64-mingw32\lib\libSDL2.dll.a и копируем его в RUST_PATH\bin\rustlib\TARGET-pc-windows-gnu\lib\
- качаем [SDL2-2.0.3-win32-x86.zip](https://www.libsdl.org/release/SDL2-2.0.3-win32-x86.zip) или [SDL2-2.0.3-win32-x64.zip](https://www.libsdl.org/release/SDL2-2.0.3-win32-x64.zip) в зависимости от платформы, распаковываем и кладем либо в директорию, которая находится в PATH, либо рядом с exe который получится после компиляции проекта
- качаем [SDL2_image-devel-2.0.0-mingw.tar.gz](https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.0-mingw.tar.gz), распаковываем, ищем файл SDL2_image-2.0.0\TARGET-w64-mingw32\lib\libSDL2_image.dll.a и копируем его в RUST_PATH\bin\rustlib\TARGET-pc-windows-gnu\lib\
- качаем [SDL2_image-2.0.0-win32-x86.zip](https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.0-win32-x86.zip) или [SDL2_image-2.0.0-win32-x64.zip](https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.0-win32-x64.zip) в зависимости от платформы, распаковываем и кладем либо в директорию, которая находится в PATH, либо рядом с exe который получится после компиляции проекта

Сборка и запуск
--
Из корня проекта выполняем:
```Shell
cargo build --release
```
Если нужно сразу скопилировать и запустить, то так:
```Shell
cargo run --release
```


Скриншоты
--

[v0.1](https://github.com/ReanGD/rust-software-render/tree/v01) ([статья](http://reangdblog.blogspot.com/2015/08/software-render-rust.html)):

16 тыс. случайных треугольников на экране и всего 1 fps:
![Растеризация на плоскости](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_1.png "Растеризация на плоскости")

[v0.2](https://github.com/ReanGD/rust-software-render/tree/v02) ([статья](http://reangdblog.blogspot.com/2015/09/software-render-rust-3d.html)):

Низкополигональная модель освещение даже не повершинное, а пограневое, чуть более 2 млн triangle per second:
![Кольцо и пограневое освещение](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_2_1.png "Кольцо и пограневое освещение")

Высокополигональная модель освещение такое же, tps = 3.3 млн:
![Высокополигональный монстр](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_2_2.png "Высокополигональный монстр")

[v0.3](https://github.com/ReanGD/rust-software-render/tree/v03) ([статья](http://reangdblog.blogspot.com/2015/09/software-render-rust.html)):

Шар с освещением по упрощенному [Cook-Torrance](http://www.gamedev.ru/code/articles/Cook-Torrance):
![Шар - Cook-Torrance](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_3_1.png "Шар - Cook-Torrance")

Кольцо, освещение по модели Фонга-Блина:
![Кольцо - Фонг-Блин](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_3_2.png "Кольцо - Фонг-Блин")
