# projektna_prog_2
Lovro Zupan Škrlj, Matevž Jaušovec

Za projektno nalogo pri Programinju 2 bova naredila igrico v Rustu, ki bo Poker Texas hold'em. 
Naredila jo bova verjetno z rust knjižnico za delo z grafikami SDL2. Projekt ne bi smel biti prelahek, saj ga lahko
po poterbi zakompliciramo kakor hočemo z programiranjem nasprotnikov(botov), ki bi lahko igrali poker popolnoma
matematično pravilno, če bo pa še to prelahko pa se lahko tudi
vzpostavi server na katerem bo možno igrati igrico proti drugim igralcem.

# Usage
## 1. Namestitev

### 1.1. Namestitev SDL2 knjižnic
Odprite terminal in zaženite na linux debian:
```bash
sudo apt update
sudo apt install -y \
  libsdl2-dev \          # Osnovna SDL2 knjižnica
  libsdl2-image-dev \    # Podpora za slike (PNG, JPG itd.)
  libsdl2-ttf-dev        # Podpora za TrueType font
```
Na macu podobno lahko (treba pogooglat).
Namestitev SDL2 knjižnice na windowsih je nekoliko bolj zakomplicirana. Nalodila za namestitev se nahajajo na 
[https://github.com/Rust-SDL2/rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2)

### 1.2. Sauron app

Za izgraditev projekta Sauron app je potreben wasm-pack:
```bash
cargo install wasm-pack
```
Za zagon aplikacije Sauron app je potreben osnovni HTTP server:
```bash
cargo install basic-http-server
```


## 2. Grajenje projekta
Imamo 3 frontende, prvi za sdl2 app, drugi za terminal app in tretji za sauron app.

Terminal app:
```bash
cargo run --bin terminal_app
```

Sdl2 app:
```bash
cargo run --bin sdl2_app --features run_with_sdl2
```

Sauron app:
* grajenje projekta:
```bash
wasm-pack build --release --target=web --features compile_with_sauron
```
* zagon serverja:
```bash
basic-http-server
```
