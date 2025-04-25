# projektna_prog_2
Lovro Zupan Škrlj, Matevž Jaušovec

Za projektno nalogo pri Programinju 2 bova naredila igrico v Rustu, ki bo Poker Texas hold'em. 
Naredila jo bova verjetno z rust knjižnico za delo z grafikami SDL2. Projekt ne bi smel biti prelahek, saj ga lahko
po poterbi zakompliciramo kakor hočemo z programiranjem nasprotnikov(botov), ki bi lahko igrali poker popolnoma
matematično pravilno, če bo pa še to prelahko pa se lahko tudi
vzpostavi server na katerem bo možno igrati igrico proti drugim igralcem.

# Usage
## 1. Namestitev SDL2 knjižnic
Odprite terminal in zaženite:
```bash
sudo apt update
sudo apt install -y \
  libsdl2-dev \          # Osnovna SDL2 knjižnica
  libsdl2-image-dev \    # Podpora za slike (PNG, JPG itd.)
  libsdl2-ttf-dev        # Podpora za TrueType font
```

## 2. Grajenje projekta
Imamo 2 frontenda, prvi za sdl2 app, drugi za terminal app, sauron web app je še v nastajanju. Vse je še v nastajanju.

Terminal app:
cargo run --bin terminal_app

Sdl2 app:
cargo run --bin sdl2_app --features run_with_sdl2