# ScholaGraeca
Hoc parvo ludo litteras Graecas disce\!

-----

**Windows** : Visual Studio (cum "Développement Desktop C++") vel MinGW.

**Linux** : `sudo apt install cmake g++ libglfw3-dev libgl1-mesa-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev`

**Mac** : `brew install cmake` (Xcode installatum esse debet).

### Compilatio (Linea Iussorum)

Aperi terminale tuum in directorio quod `main.cpp` et `CMakeLists.txt` continet.

Si absens, crea directorium constructionis addeque `font.ttf` (eodem ut vis, arial do in inceptum istum):

```bash
mkdir build
cd build
```

Genera inceptum :

```bash
cmake ..
```

Compila :

```bash
cmake --build .
```

*(In Windows cum Visual Studio, hoc fasciculum .sln generat quem aperire potes, vel in modo "Release" compilare potes per iussum supra).*

### Executio

1.  Cura ut fasciculus `font.ttf` in eodem directorio sit atque executabile generatum (saepe in `build/Debug/` sub Windows vel `build/` sub Linux).

2.  Exsequere programma :

    ```bash
    ./ScholaGraeca
    ```

    (vel `ScholaGraeca.exe`).

-----

### Nota terminologiae (Lexicon)

Accomodationibus necessariis ad vocabula technica hodierna: 

  * **Terminale** quod interpretatur (linguam fracogallicam) est Terminal
  * **Directorium** quod interpretatur est Répertoire
  * **Fasciculus** quod interpretatur est Fichier
  * **Inceptum** quod interpretatur est Projet
  * **Compilatio** quod interpretatur est Compilation
