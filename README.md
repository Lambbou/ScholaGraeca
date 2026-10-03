# ScholaGraeca

**Hoc ludo litteras, verba et mythologiam Graecam per sermonem Latinum disce!**

*ScholaGraeca* nunc est ludus electronicus bipartitus (2D RPG) scriptus in lingua **Rust** ope motoris **Bevy**. Heros naufragus in litore Epiri eiectus terras Helladis peragrat, cum incolis Latine conloquitur, et monstra mythologica Graecis quaestionibus debellat.

-----

### Principia Ludi (Univers & Mécaniques)

1. **Omnia Latine** : Tota graphica interfacies, status herois, dialogi et mandata solum in lingua Latina exarantur.
2. **Certamina Graece** : Impetus in monstra mythologica (*Harpyiam*, *Aprum Calydonium*, *Sphingem*, *Minotaurum*) per quaestiones de litteris et verbis Graecis decernuntur. Series responsorum rectorum (*Streak*) damnum multiplicat!
3. **Gradus Civici** : Experientia (XP) herois gradum auget :
   * **Apothetes** (Naufragus in harenis eiectus)
   * **Ephebus** (Tiro chlamyde et hasta instructus)
   * **Polites** (Civis togatus xipho armatus)
   * **Philosophus** (Vir sapiens papyro munitus)
   * **Archon** (Magistratus supremus auro et lauro ornatus)
4. **Mutatio Aspectus** : Ad quemque novum gradum, herois imago (*sprite*) pulchrior et clarior apparet.

-----

### Praerequisita & Constructio (Installation)

#### 1. Praerequisita Systematis (Linux Ubuntu/Debian)

Ad necessarias bibliothecas soni (ALSA) et graphicas ac compilatorem celerem (`lld`) installandum, hoc scriptum cum potestate administratoris (*sudo*) exsequere :

```bash
sudo bash setup_prerequisites.sh
```

*(Instrumentum Rust ac Cargo installatum esse oportet : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).*

#### 2. Compilatio et Executio per Cargo

Aperi terminale in directorio incepti :

```bash
# Compilatio et executio ludi
cargo run
```

Pro probationibus automatis :
```bash
cargo test
```

-----

### Gubernatio (Contrôles du jeu)

* **Cursus / Motus** : `Z`, `Q`, `S`, `D` vel `W`, `A`, `S`, `D` vel sagittae directoriae (`↑`, `↓`, `←`, `→`).
* **Conloqui / Certare** : Clavis `E` cum prope personam (PNJ) aut monstrum adstas.
* **Introire in ludum** : In indice tituli, preme murem super botone `INCIPE CURSUM`.

-----

### Structura Fasciculorum

* `assets/data/` : Fasciculi JSON cum alphabeto Graeco, lexico, monstris, fabulis antiquis et quaestionibus.
* `assets/textures/` : Texturae et personarum imagines graphicae (formae herois, incolae, monstra, saxa, mare).
* `assets/fonts/` : Litterarum typi antiqui (`ancient.ttf`).
* `src/domain/` : Logica ludi (gradus civici, status herois, examen sermonis Latini, certamina Graeca).
* `src/overworld/` : Tabula geographica, motus herois, camera et interfacies status (HUD).

-----

### Nota terminologiae (Lexicon)

Accomodationibus necessariis ad vocabula technica hodierna :
* **Terminale** quod interpretatur est *Terminal*
* **Directorium** quod interpretatur est *Répertoire / Dossier*
* **Fasciculus** quod interpretatur est *Fichier*
* **Inceptum** quod interpretatur est *Projet*
* **Compilatio** quod interpretatur est *Compilation*
* **Motor ludi** quod interpretatur est *Moteur de jeu (Game Engine)*
