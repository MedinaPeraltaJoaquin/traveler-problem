# Proyecto 1: Problema del agente viajero

Este proyecto está escrito en **Rust** y utiliza **SQLite** como base de datos.  
Incluye un ejecutable que admite varias opciones desde la línea de comandos para procesamiento de archivos `.tsp`, generación de semillas y salida en formato SVG.

[Reporte del proyecto](./Proyecto1_Heuristicas.pdf)

---

## 📦 Requisitos

Antes de compilar y correr el proyecto, asegúrate de tener instalados los siguientes programas:

### 1. Rust y Cargo
Instala el *toolchain* oficial de Rust que incluye `cargo` (el gestor de paquetes y compilación):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Verifica la instalación:
```bash
rustc --version
cargo --version
```

### 2. Sqlite3 
Instala el sistema de base de datos de `sqlite3`, en sistema linux o mac respectivamente:
```bash
sudo apt-get install sqlite3 libsqlite3-dev
brew install sqlite
```

---

## Construcción del proyecto.
Para esto, clona este repositorio y entra en la carpeta del proyecto:
```bash
git clone https://github.com/MedinaPeraltaJoaquin/traveler-problem
cd traveler-problem
```
Compila en modo debug:
```bash
cargo build --release
```

---

## Ejecución del proyecto
Para esto, corre el programa con el siguiente comando:
```bash
cargo run -- <opciones>
./target/debug/traveler-problem <opciones>
```
Indica `--help` o `-h` para mostrar el menú:
```bash
Uso: programa [opciones]

Opciones:
  -h, --help         Muestra esta ayuda y termina
  -v                 Activa el modo verbose
  -p <path>          Ruta explícita como lista de enteros o archivo .tsp
  -s <I> <F>         Genera semillas en el rango [I, F]
  -s <n>             Inicializa con la semilla n
  -rs <n>            Genera n semillas aleatorias
  -svg               Activa el modo de salida SVG
```

---

##  Notas adicionales

Debe de existir una base de datos `tsp` para poder correr el sistema, y además debe de existir un archivo
`.env` en la raíz del proyecto, con las siguientes caracteristicas:
```bash
SIZE_LOTE=usize               <tamaño del lote>
DATABASE_URL=String           <dirección en memoria de la base de datos>
SQL_PATH=String               <dirección en memoria del archivo .sql de la base de datos>
E_S=f64                       <valor de epsilon para la heuristica del recocido simulado>
E_P=f64                       <valor de la epsilon de la heuristica de temperatura>
PERCENTAGE=f64                <porcentaje de aceptación>
LIMIT=usize                   <limite de iteraciones>
N=usize                       <número de iteraciones para la aceptación de heuristica de temperatura>
```



