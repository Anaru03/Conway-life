# Conway's Game of Life

Implementacion del Juego de la Vida de Conway en Rust, usando un framebuffer propio y una ventana en tiempo real con [`minifb`](https://crates.io/crates/minifb).

## Demo

![Conway's Game of Life corriendo](Conway_life_.gif)

## Reglas implementadas

1. Underpopulation: una celula viva con menos de 2 vecinos vivos, muere.
2. Survival: una celula viva con 2 o 3 vecinos vivos, sobrevive.
3. Overpopulation: una celula viva con mas de 3 vecinos vivos, muere.
4. Reproduction: una celula muerta con exactamente 3 vecinos vivos, nace.

## Patron inicial

El patron inicial combina:
- **Still lifes**: block, beehive, loaf, boat, tub
- **Osciladores**: blinker, toad, beacon, pulsar, pentadecathlon
- **Spaceships**: gliders y lightweight spaceships (LWSS), colocados en carriles despejados para que puedan viajar por el tablero

## Como correrlo

```bash
cargo run --release
```

Se abre una ventana con el tablero de 100x100 escalado x6. Presiona `Esc` o cierra la ventana para salir (al cerrar tambien se guarda el ultimo frame como `out.bmp`).

## Estructura del proyecto

- `framebuffer.rs`: buffer de pixeles con `set_pixel`, `get_color` y guardado a imagen.
- `patterns.rs`: definicion de los organismos clasicos como listas de coordenadas.
- `render.rs`: `init()` arma el patron inicial, `step()` aplica las reglas de Conway usando un snapshot de la generacion anterior.
- `main.rs`: crea la ventana, hace el loop de render y actualiza cada frame.