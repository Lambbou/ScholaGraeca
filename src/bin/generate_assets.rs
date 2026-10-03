use image::{Rgba, RgbaImage};
use std::fs;
use std::path::Path;

fn set_pixel_safe(img: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
        img.put_pixel(x as u32, y as u32, color);
    }
}

fn fill_rect(img: &mut RgbaImage, x0: i32, y0: i32, w: i32, h: i32, color: Rgba<u8>) {
    for y in y0..(y0 + h) {
        for x in x0..(x0 + w) {
            set_pixel_safe(img, x, y, color);
        }
    }
}

// Couleurs antiques
const C_TRANSPARENT: Rgba<u8> = Rgba([0, 0, 0, 0]);
const C_SKIN: Rgba<u8> = Rgba([230, 185, 140, 255]);
const C_SKIN_SHADOW: Rgba<u8> = Rgba([200, 150, 110, 255]);
const C_HAIR_BROWN: Rgba<u8> = Rgba([80, 50, 30, 255]);
const C_HAIR_GREY: Rgba<u8> = Rgba([180, 180, 190, 255]);
const C_EYE_BLACK: Rgba<u8> = Rgba([30, 25, 25, 255]);

// Couleurs vêtements
const C_RAGS: Rgba<u8> = Rgba([160, 130, 95, 255]);
const C_CHLAMYS_WHITE: Rgba<u8> = Rgba([240, 240, 245, 255]);
const C_AEGEAN_BLUE: Rgba<u8> = Rgba([35, 110, 190, 255]);
const C_PURPLE: Rgba<u8> = Rgba([120, 25, 75, 255]);
const C_BRONZE: Rgba<u8> = Rgba([205, 145, 45, 255]);
const C_BRONZE_DARK: Rgba<u8> = Rgba([140, 95, 25, 255]);
const C_GOLD: Rgba<u8> = Rgba([255, 215, 0, 255]);
const C_RED_CREST: Rgba<u8> = Rgba([190, 30, 30, 255]);

// Tuiles d'environnement
const C_WATER_DEEP: Rgba<u8> = Rgba([25, 70, 150, 255]);
const C_WATER_MID: Rgba<u8> = Rgba([40, 115, 195, 255]);
const C_WATER_SHALLOW: Rgba<u8> = Rgba([65, 165, 220, 255]);
const C_SAND: Rgba<u8> = Rgba([235, 210, 150, 255]);
const C_SAND_DARK: Rgba<u8> = Rgba([215, 190, 130, 255]);
const C_GRASS: Rgba<u8> = Rgba([110, 165, 75, 255]);
const C_GRASS_DARK: Rgba<u8> = Rgba([85, 135, 55, 255]);
const C_ROCK: Rgba<u8> = Rgba([140, 135, 130, 255]);
const C_ROCK_DARK: Rgba<u8> = Rgba([100, 95, 90, 255]);
const C_STONE_PATH: Rgba<u8> = Rgba([195, 190, 180, 255]);
const C_MARBLE: Rgba<u8> = Rgba([245, 245, 240, 255]);
const C_MARBLE_SHADOW: Rgba<u8> = Rgba([190, 190, 185, 255]);

fn generate_hero_base(img: &mut RgbaImage, hair_color: Rgba<u8>) {
    // Tête
    fill_rect(img, 11, 8, 10, 10, C_SKIN);
    // Cheveux
    fill_rect(img, 10, 5, 12, 5, hair_color);
    fill_rect(img, 9, 8, 3, 5, hair_color);
    fill_rect(img, 20, 8, 3, 5, hair_color);
    // Yeux
    set_pixel_safe(img, 13, 12, C_EYE_BLACK);
    set_pixel_safe(img, 18, 12, C_EYE_BLACK);
    // Cou
    fill_rect(img, 14, 18, 4, 3, C_SKIN_SHADOW);
}

fn create_hero_apothetes() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    generate_hero_base(&mut img, C_HAIR_BROWN);
    
    // Corps dénudé de naufragé
    fill_rect(&mut img, 11, 21, 10, 9, C_SKIN);
    // Bras nus
    fill_rect(&mut img, 8, 21, 3, 11, C_SKIN);
    fill_rect(&mut img, 21, 21, 3, 11, C_SKIN);
    // Haillons déchirés (pagne)
    fill_rect(&mut img, 10, 29, 12, 7, C_RAGS);
    set_pixel_safe(&mut img, 11, 36, C_RAGS);
    set_pixel_safe(&mut img, 14, 36, C_RAGS);
    set_pixel_safe(&mut img, 17, 36, C_RAGS);
    set_pixel_safe(&mut img, 20, 36, C_RAGS);
    // Jambes nues & pieds
    fill_rect(&mut img, 11, 36, 4, 8, C_SKIN);
    fill_rect(&mut img, 17, 36, 4, 8, C_SKIN);
    
    img
}

fn create_hero_ephebus() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    generate_hero_base(&mut img, C_HAIR_BROWN);
    
    // Chlamyde d'éphèbe (blanche avec liseré bleu égéen)
    fill_rect(&mut img, 10, 21, 12, 13, C_CHLAMYS_WHITE);
    fill_rect(&mut img, 10, 32, 12, 2, C_AEGEAN_BLUE);
    // Bras et lance de bronze
    fill_rect(&mut img, 8, 21, 3, 10, C_SKIN);
    fill_rect(&mut img, 21, 21, 3, 10, C_SKIN);
    // Lance dans la main droite
    fill_rect(&mut img, 25, 4, 2, 38, C_BRONZE_DARK);
    fill_rect(&mut img, 24, 2, 4, 4, C_BRONZE);
    // Sandales en cuir
    fill_rect(&mut img, 11, 35, 4, 9, C_SKIN);
    fill_rect(&mut img, 17, 35, 4, 9, C_SKIN);
    fill_rect(&mut img, 11, 41, 4, 3, C_RAGS);
    fill_rect(&mut img, 17, 41, 4, 3, C_RAGS);
    
    img
}

fn create_hero_polites() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    generate_hero_base(&mut img, C_HAIR_BROWN);
    
    // Toge grecque blanche plissée avec bande pourpre citoyenne
    fill_rect(&mut img, 9, 21, 14, 18, C_CHLAMYS_WHITE);
    fill_rect(&mut img, 9, 24, 14, 3, C_PURPLE);
    fill_rect(&mut img, 13, 21, 4, 18, C_PURPLE);
    // Épée courte grecque (xiphos) au côté
    fill_rect(&mut img, 6, 28, 3, 8, C_BRONZE);
    fill_rect(&mut img, 5, 27, 5, 2, C_GOLD);
    // Sandales citoyennes
    fill_rect(&mut img, 11, 40, 4, 4, C_BRONZE_DARK);
    fill_rect(&mut img, 17, 40, 4, 4, C_BRONZE_DARK);
    
    img
}

fn create_hero_philosophus() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    generate_hero_base(&mut img, C_HAIR_GREY);
    // Barbe de philosophe
    fill_rect(&mut img, 12, 17, 8, 6, C_HAIR_GREY);
    
    // Longue tunique d'érudit pourpre
    fill_rect(&mut img, 8, 21, 16, 21, C_PURPLE);
    fill_rect(&mut img, 8, 38, 16, 4, C_GOLD);
    // Rouleau de papyrus dans la main gauche
    fill_rect(&mut img, 5, 28, 4, 8, C_SAND);
    fill_rect(&mut img, 5, 29, 4, 1, C_HAIR_BROWN);
    fill_rect(&mut img, 5, 33, 4, 1, C_HAIR_BROWN);
    // Bâton de marche dans la main droite
    fill_rect(&mut img, 24, 15, 2, 27, C_HAIR_BROWN);
    
    img
}

fn create_hero_archon() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    // Casque hoplite de bronze
    fill_rect(&mut img, 10, 5, 12, 12, C_BRONZE);
    fill_rect(&mut img, 9, 2, 14, 4, C_RED_CREST);
    // Couronne de laurier dorée
    fill_rect(&mut img, 9, 7, 14, 2, C_GOLD);
    // Visage casqué
    fill_rect(&mut img, 13, 11, 6, 5, C_SKIN);
    set_pixel_safe(&mut img, 14, 13, C_EYE_BLACK);
    set_pixel_safe(&mut img, 17, 13, C_EYE_BLACK);
    
    // Cuirasse dorée anatomique
    fill_rect(&mut img, 8, 19, 16, 14, C_GOLD);
    fill_rect(&mut img, 10, 21, 12, 10, C_BRONZE);
    // Jupe de combat hoplite (ptéryges)
    fill_rect(&mut img, 9, 33, 14, 5, C_RED_CREST);
    // Jambières en bronze (cnémides)
    fill_rect(&mut img, 10, 38, 4, 6, C_GOLD);
    fill_rect(&mut img, 18, 38, 4, 6, C_GOLD);
    // Bouclier rond (aspis) à gauche
    fill_rect(&mut img, 2, 22, 6, 14, C_GOLD);
    fill_rect(&mut img, 4, 24, 2, 10, C_AEGEAN_BLUE);
    
    img
}

fn create_npc_piscator() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    // Calotte de marin / chapeau de paille
    fill_rect(&mut img, 9, 4, 14, 4, C_SAND_DARK);
    fill_rect(&mut img, 11, 8, 10, 9, C_SKIN_SHADOW);
    fill_rect(&mut img, 11, 15, 10, 6, C_HAIR_GREY); // Barbe blanche
    set_pixel_safe(&mut img, 13, 11, C_EYE_BLACK);
    set_pixel_safe(&mut img, 18, 11, C_EYE_BLACK);
    
    // Tunique bleue de pêcheur
    fill_rect(&mut img, 9, 20, 14, 16, C_AEGEAN_BLUE);
    // Filet de pêche sur l'épaule
    for y in 22..36 {
        for x in (11..21).step_by(3) {
            set_pixel_safe(&mut img, x, y, C_SAND);
        }
    }
    fill_rect(&mut img, 11, 37, 4, 6, C_SKIN_SHADOW);
    fill_rect(&mut img, 17, 37, 4, 6, C_SKIN_SHADOW);
    
    img
}

fn create_monster_harpyia() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(48, 48, C_TRANSPARENT);
    // Ailes déployées
    fill_rect(&mut img, 4, 10, 12, 22, C_PURPLE);
    fill_rect(&mut img, 32, 10, 12, 22, C_PURPLE);
    fill_rect(&mut img, 2, 14, 6, 16, C_HAIR_BROWN);
    fill_rect(&mut img, 40, 14, 6, 16, C_HAIR_BROWN);
    
    // Tête humaine / cheveux sauvages
    fill_rect(&mut img, 18, 6, 12, 10, C_SKIN);
    fill_rect(&mut img, 16, 4, 16, 5, C_HAIR_BROWN);
    fill_rect(&mut img, 14, 8, 4, 10, C_HAIR_BROWN);
    fill_rect(&mut img, 30, 8, 4, 10, C_HAIR_BROWN);
    set_pixel_safe(&mut img, 21, 10, C_RED_CREST); // Yeux féroces
    set_pixel_safe(&mut img, 26, 10, C_RED_CREST);
    
    // Buste et serres d'oiseau
    fill_rect(&mut img, 19, 16, 10, 14, C_HAIR_BROWN);
    fill_rect(&mut img, 18, 30, 4, 12, C_BRONZE);
    fill_rect(&mut img, 26, 30, 4, 12, C_BRONZE);
    // Serres acérées
    fill_rect(&mut img, 16, 40, 6, 4, C_GOLD);
    fill_rect(&mut img, 26, 40, 6, 4, C_GOLD);
    
    img
}

// Génération des tuiles
fn create_tile_water() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 32, C_WATER_DEEP);
    for y in (4..32).step_by(8) {
        fill_rect(&mut img, 2, y, 12, 2, C_WATER_MID);
        fill_rect(&mut img, 16, y + 4, 14, 2, C_WATER_SHALLOW);
    }
    img
}

fn create_tile_sand() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 32, C_SAND);
    for y in (0..32).step_by(4) {
        for x in (0..32).step_by(6) {
            set_pixel_safe(&mut img, (x + (y % 3)) % 32, y, C_SAND_DARK);
        }
    }
    img
}

fn create_tile_grass() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 32, C_GRASS);
    for y in (2..32).step_by(6) {
        for x in (3..32).step_by(7) {
            fill_rect(&mut img, x, y, 2, 3, C_GRASS_DARK);
        }
    }
    img
}

fn create_tile_rock() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 32, C_ROCK);
    fill_rect(&mut img, 4, 4, 24, 24, C_ROCK_DARK);
    fill_rect(&mut img, 8, 8, 16, 16, C_ROCK);
    img
}

fn create_tile_stonepath() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 32, C_STONE_PATH);
    for y in (0..32).step_by(8) {
        fill_rect(&mut img, 0, y, 32, 1, C_ROCK_DARK);
        for x in (0..32).step_by(16) {
            fill_rect(&mut img, (x + y * 2) % 32, y, 1, 8, C_ROCK_DARK);
        }
    }
    img
}

fn create_prop_column() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(32, 48, C_TRANSPARENT);
    // Chapiteau dorique
    fill_rect(&mut img, 6, 4, 20, 6, C_MARBLE);
    fill_rect(&mut img, 8, 8, 16, 2, C_MARBLE_SHADOW);
    // Fût cannelé
    fill_rect(&mut img, 10, 10, 12, 30, C_MARBLE);
    for x in (11..21).step_by(3) {
        fill_rect(&mut img, x, 10, 1, 30, C_MARBLE_SHADOW);
    }
    // Base
    fill_rect(&mut img, 6, 40, 20, 6, C_MARBLE);
    fill_rect(&mut img, 4, 44, 24, 3, C_MARBLE_SHADOW);
    img
}

fn main() {
    let dir = Path::new("assets/textures");
    fs::create_dir_all(dir).unwrap();

    println!("Generantur texturis pro ScholaGraeca...");
    create_hero_apothetes().save(dir.join("hero_apothetes.png")).unwrap();
    create_hero_ephebus().save(dir.join("hero_ephebus.png")).unwrap();
    create_hero_polites().save(dir.join("hero_polites.png")).unwrap();
    create_hero_philosophus().save(dir.join("hero_philosophus.png")).unwrap();
    create_hero_archon().save(dir.join("hero_archon.png")).unwrap();

    create_npc_piscator().save(dir.join("npc_piscator.png")).unwrap();
    create_monster_harpyia().save(dir.join("monster_harpyia.png")).unwrap();

    create_tile_water().save(dir.join("tile_water.png")).unwrap();
    create_tile_sand().save(dir.join("tile_sand.png")).unwrap();
    create_tile_grass().save(dir.join("tile_grass.png")).unwrap();
    create_tile_rock().save(dir.join("tile_rock.png")).unwrap();
    create_tile_stonepath().save(dir.join("tile_stonepath.png")).unwrap();
    create_prop_column().save(dir.join("prop_column.png")).unwrap();

    println!("Omnes texturae feliciter creatae sunt in assets/textures/!");
}
