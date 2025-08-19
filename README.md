# Projektna naloga za programiranje 2 - POKER
Lovro Zupan Škrlj, Matevž Jaušovec

Za projektno nalogo pri Programinju 2 sva naredila igrico v Rustu, ki je Poker Texas hold'em. 
Naredila sva jo z rust knjižnico za delo z grafikami SDL2, ki naredijo "desktop-app", in pa ker je SDL2 zahtevno naložiti na sistem Windows, sva naredila še "web" verzijo, ki je narejena z knižnico Sauron.
Nasprotniki proti katerim uporabnik igra so boti, ki igrajo na podlagi tega kako dobre karte so dobili. Na SDL2-app pa je implementirano tudi shranjevanje igre, torej če je uporabnik premagal bote in veliko
zasložil si lahko shrani igro, in nadaljuje to isto igro kasneje.

# Usage
## 1. Namestitev

### 1.1. Namestitev SDL2 knjižnic
Odprite terminal in zaženite na linux debian:
```bash
sudo apt update
sudo apt install -y \
  libsdl2-dev \          # Osnovna SDL2 knjižnica
  libsdl2-image-dev \    # Podpora za slike (PNG, JPG itd.)
  libsdl2-ttf-dev \
  libsdl2-gfx-dev    # Podpora za TrueType font
```
Na MacOs sistemu podobno lahko namestimo SLD2 knjižnice v parih komandah, ampak ker nimava MacOs, nisva dala navodil za 
namestitve saj ne bi mogla preveriti če dela.
Namestitev SDL2 knjižnice na windowsih je nekoliko bolj zakomplicirana. Nalodila za namestitev za vse podprte sisteme se nahajajo na 
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
Imamo 3 aplikacije, prva za sdl2 app, druga za terminal app, ki sva jo uprabljala za razvijanje logike, in tretja za sauron app.

Terminal app (za razvijanje logike):
```bash
cargo run --bin terminal_app
```

Sdl2 desktop app:
```bash
cargo run --bin sdl2_app --features run_with_sdl2
```

Sauron web app:
* grajenje projekta:
```bash
wasm-pack build --release --target=web --features compile_with_sauron
```
* zagon serverja:
```bash
basic-http-server
```
