#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <vector>
#include <string>
#include <random>
#include <algorithm>
#include <iostream>

// ---------------------------------------------------------
// DONNÉES (DATA)
// ---------------------------------------------------------

struct Littera {
    std::string majuscula; // Majuscule (ex: A)
    std::string minuscula; // Minuscule (ex: α)
    std::string nomen;     // Nom (ex: Alpha)
    std::string trans;     // Translittération (ex: A)
};

struct Verbum {
    std::string graecum_maj; // Mot grec MAJ (ex: LOGOS)
    std::string graecum_min; // Mot grec min (ex: logos)
    std::string latinum;     // Translittération latine
};

// Alphabet complet (Majuscules + Minuscules)
const std::vector<Littera> ALPHABETUM = {
    { u8"Α", u8"α", "Alpha", "A" },   { u8"Β", u8"β", "Beta", "B" },
    { u8"Γ", u8"γ", "Gamma", "G" },   { u8"Δ", u8"δ", "Delta", "D" },
    { u8"Ε", u8"ε", "Epsilon", "E" }, { u8"Ζ", u8"ζ", "Zeta", "Z" },
    { u8"Η", u8"η", "Eta", "E (longus)"}, { u8"Θ", u8"θ", "Theta", "TH" },
    { u8"Ι", u8"ι", "Iota", "I" },    { u8"Κ", u8"κ", "Kappa", "K" },
    { u8"Λ", u8"λ", "Lambda", "L" },  { u8"Μ", u8"μ", "Mu", "M" },
    { u8"Ν", u8"ν", "Nu", "N" },      { u8"Ξ", u8"ξ", "Xi", "X" },
    { u8"Ο", u8"ο", "Omicron", "O" }, { u8"Π", u8"π", "Pi", "P" },
    { u8"Ρ", u8"ρ", "Rho", "R" },     { u8"Σ", u8"σ", "Sigma", "S" }, // Sigma standard
    { u8"Τ", u8"τ", "Tau", "T" },     { u8"Υ", u8"υ", "Upsilon", "Y/U" },
    { u8"Φ", u8"φ", "Phi", "PH" },    { u8"Χ", u8"χ", "Chi", "CH" },
    { u8"Ψ", u8"ψ", "Psi", "PS" },    { u8"Ω", u8"ω", "Omega", "O (longus)" }
};

// Inventaire de mots avec gestion du Sigma final (ς)
const std::vector<Verbum> VERBA = {
    // --- A ---
    { u8"ΑΓΑΠΗ",       u8"αγαπη",       "AGAPE (Amor)" },
    { u8"ΑΓΓΕΛΟΣ",     u8"αγγελος",     "ANGELOS (Angelus)" },
    { u8"ΑΓΟΡΑ",       u8"αγορα",       "AGORA (Forum)" },
    { u8"ΑΕΡΟΔΡΟΜΙΟΝ", u8"αεροδρομιον", "AERODROMION (Aeroportus)" }, // Neo-Latin
    { u8"ΑΘΛΗΤΗΣ",     u8"αθλητης",     "ATHLETES (Athleta)" },
    { u8"ΑΙΝΙΓΜΑ",     u8"αινιγμα",     "ENIGMA (Aenigma)" },
    { u8"ΑΚΑΔΗΜΙΑ",    u8"ακαδημια",    "ACADEMIA" },
    { u8"ΑΚΡΟΠΟΛΙΣ",   u8"ακροπολις",   "ACROPOLIS (Arx)" },
    { u8"ΑΛΗΘΕΙΑ",     u8"αληθεια",     "ALETHEIA (Veritas)" },
    { u8"ΑΛΦΑΒΗΤΟΝ",   u8"αλφαβητον",   "ALPHABETON (Alphabetum)" },
    { u8"ΑΝΑΛΥΣΙΣ",    u8"αναλυσις",    "ANALYSIS (Resolutio)" },
    { u8"ΑΝΑΡΧΙΑ",     u8"αναρχια",     "ANARCHIA" },
    { u8"ΑΝΘΡΩΠΟΣ",    u8"ανθρωπος",    "ANTHROPOS (Homo)" },
    { u8"ΑΡΙΘΜΟΣ",     u8"αριθμος",     "ARITHMOS (Numerus)" },
    { u8"ΑΡΜΟΝΙΑ",     u8"αρμονια",     "HARMONIA (Concordia)" },
    { u8"ΑΣΤΡΟΝΑΥΤΗΣ", u8"αστροναυτης", "ASTRONAUTES (Astronauta)" },
    { u8"ΑΣΤΡΟΝ",      u8"αστρον",      "ASTRON (Astrum/Stella)" },
    { u8"ΑΤΟΜΟΣ",      u8"ατομος",      "ATOMOS (Individuum)" },
    { u8"ΑΥΤΟΝΟΜΙΑ",   u8"αυτονομια",   "AUTONOMIA" },
    { u8"ΑΤΛΑΣ",       u8"ατλας",       "ATLAS" },

    // --- B, G, D ---
    { u8"ΒΑΡΒΑΡΟΣ",    u8"βαρβαρος",    "BARBAROS (Barbarus)" },
    { u8"ΒΑΣΙΛΕΥΣ",    u8"βασιλευς",    "BASILEUS (Rex)" },
    { u8"ΒΙΒΛΙΟΝ",     u8"βιβλιον",     "BIBLION (Liber)" },
    { u8"ΒΙΟΣ",        u8"βιος",        "BIOS (Vita)" },
    { u8"ΓΑΛΑΞΙΑΣ",    u8"γαλαξιας",    "GALAXIAS (Via Lactea)" },
    { u8"ΓΕΝΕΣΙΣ",     u8"γενεσις",     "GENESIS (Origo)" },
    { u8"ΓΕΩΜΕΤΡΙΑ",   u8"γεωμετρια",   "GEOMETRIA" },
    { u8"ΓΙΓΑΣ",       u8"γιγας",       "GIGAS (Gigas)" },
    { u8"ΓΥΜΝΑΣΙΟΝ",   u8"γυμνασιον",   "GYMNASION (Gymnasium)" },
    { u8"ΔΑΙΜΩΝ",      u8"δαιμων",      "DAIMON (Daemon/Spiritus)" },
    { u8"ΔΗΜΟΚΡΑΤΙΑ",  u8"δημοκρατια",  "DEMOKRATIA (Res Publica)" },
    { u8"ΔΙΑΛΟΓΟΣ",    u8"διαλογος",    "DIALOGOS (Colloquium)" },
    { u8"ΔΙΠΛΩΜΑ",     u8"διπλωμα",     "DIPLOMA" },
    { u8"ΔΡΑΚΩΝ",      u8"δρακων",      "DRACON (Draco)" },
    { u8"ΔΡΑΜΑ",       u8"δραμα",       "DRAMA (Fabula)" },
    { u8"ΔΥΝΑΜΙΣ",     u8"δυναμις",     "DYNAMIS (Vis/Potentia)" },

    // --- E, Z, H, TH ---
    { u8"ΕΓΩ",         u8"εγω",         "EGO (Ego)" },
    { u8"ΕΙΔΩΛΟΝ",     u8"ειδωλον",     "EIDOLON (Simulacrum)" },
    { u8"ΕΙΚΩΝ",       u8"εικων",       "ICON (Imago)" },
    { u8"ΕΚΚΛΗΣΙΑ",    u8"εκκλησια",    "ECCLESIA (Coetus)" },
    { u8"ΕΞΟΔΟΣ",      u8"εξοδος",      "EXODOS (Exitus)" },
    { u8"ΕΠΙΚΕΝΤΡΟΝ",  u8"επικεντρον",  "EPICENTRON (Epicentrum)" },
    { u8"ΕΡΩΣ",        u8"ερως",        "EROS (Amor)" },
    { u8"ΕΘΟΣ",        u8"εθος",        "ETHOS (Mos/Consuetudo)" },
    { u8"ΕΥΡΗΚΑ",      u8"ευρηκα",      "EUREKA (Inveni)" },
    { u8"ΕΥΡΩΠΗ",      u8"ευρωπη",      "EUROPE (Europa)" },
    { u8"ΖΩΔΙΑΚΟΣ",    u8"ζωδιακος",    "ZODIACOS (Zodiacus)" },
    { u8"ΖΩΟΝ",        u8"ζωον",        "ZOON (Animal)" },
    { u8"ΗΛΕΚΤΡΟΝ",    u8"ηλεκτρον",    "ELECTRON (Electrum)" },
    { u8"ΗΛΙΟΣ",       u8"ηλιος",       "HELIOS (Sol)" },
    { u8"ΗΡΩΣ",        u8"ηρως",        "HEROS (Heros)" },
    { u8"ΗΧΩ",         u8"ηχω",         "ECHO (Echo)" },
    { u8"ΘΕΑΤΡΟΝ",     u8"θεατρον",     "THEATRON (Theatrum)" },
    { u8"ΘΕΟΣ",        u8"θεος",        "THEOS (Deus)" },
    { u8"ΘΕΡΑΠΕΙΑ",    u8"θεραπεια",    "THERAPIA (Curatio)" },
    { u8"ΘΕΩΡΙΑ",      u8"θεωρια",      "THEORIA (Contemplatio)" },

    // --- I, K, L, M ---
    { u8"ΙΔΕΑ",        u8"ιδεα",        "IDEA (Species)" },
    { u8"ΙΣΤΟΡΙΑ",     u8"ιστορια",     "HISTORIA" },
    { u8"ΙΠΠΟΣ",       u8"ιππος",       "HIPPOS (Equus)" },
    { u8"ΚΑΚΟΦΩΝΙΑ",   u8"κακοφωνια",   "CACOPHONIA" },
    { u8"ΚΑΝΩΝ",       u8"κανων",       "CANON (Regula)" },
    { u8"ΚΙΝΗΜΑ",      u8"κινημα",      "CINEMA (Motus)" },
    { u8"ΚΟΣΜΟΣ",      u8"κοσμος",      "COSMOS (Mundus)" },
    { u8"ΚΡΙΣΙΣ",      u8"κρισις",      "CRISIS (Iudicium)" },
    { u8"ΚΥΚΛΟΣ",      u8"κυκλος",      "CYCLOS (Circulus)" },
    { u8"ΚΩΜΩΔΙΑ",     u8"κωμωδια",     "COMOEDIA" },
    { u8"ΛΑΒΥΡΙΝΘΟΣ",  u8"λαβυρινθος",  "LABYRINTHOS (Labyrinthus)" },
    { u8"ΛΕΞΙΚΟΝ",     u8"λεξικον",     "LEXICON (Dictionarium)" },
    { u8"ΛΟΓΙΚΗ",      u8"λογικη",      "LOGICA (Ratio)" },
    { u8"ΛΟΓΟΣ",       u8"λογος",       "LOGOS (Verbum/Ratio)" },
    { u8"ΜΑΘΗΜΑ",      u8"μαθημα",      "MATHEMA (Disciplina)" },
    { u8"ΜΑΝΙΑ",       u8"μανια",       "MANIA (Insania/Furor)" },
    { u8"ΜΕΤΑΦΟΡΑ",    u8"μεταφορα",    "METAPHORA (Translatio)" },
    { u8"ΜΕΤΡΟΝ",      u8"μετρον",      "METRON (Mensura)" },
    { u8"ΜΗΧΑΝΗ",      u8"μηχανη",      "MECHANE (Machina)" },
    { u8"ΜΙΚΡΟΒΙΟΝ",   u8"μικροβιον",   "MICROBION" },
    { u8"ΜΟΥΣΕΙΟΝ",    u8"μουσειον",    "MUSEUM (Musaeum)" },
    { u8"ΜΥΘΟΣ",       u8"μυθος",       "MYTHOS (Fabula)" },

    // --- N, X, O, P ---
    { u8"ΝΑΥΤΗΣ",      u8"ναυτης",      "NAUTES (Nauta)" },
    { u8"ΝΕΚΡΩΣΙΣ",    u8"νεκρωσις",    "NECROSIS (Mors)" },
    { u8"ΝΕΚΤΑΡ",      u8"νεκταρ",      "NECTAR" },
    { u8"ΝΟΜΟΣ",       u8"νομος",       "NOMOS (Lex)" },
    { u8"ΞΕΝΟΣ",       u8"ξενος",       "XENOS (Hospes/Peregrinus)" },
    { u8"ΟΔΥΣΣΕΙΑ",    u8"οδυσσεια",    "ODYSSEA" },
    { u8"ΟΙΚΟΝΟΜΙΑ",   u8"οικονομια",   "OECONOMIA (Dispensatio)" },
    { u8"ΟΛΥΜΠΙΑ",     u8"ολυμπια",     "OLYMPIA" },
    { u8"ΟΡΓΑΝΟΝ",     u8"οργανον",     "ORGANON (Instrumentum)" },
    { u8"ΟΦΘΑΛΜΟΣ",    u8"οφθαλμος",    "OPHTHALMOS (Oculus)" },
    { u8"ΠΑΝΘΕΟΝ",     u8"πανθεον",     "PANTHEON" },
    { u8"ΠΑΡΑΔΕΙΓΜΑ",  u8"παραδειγμα",  "PARADIGMA (Exemplum)" },
    { u8"ΠΑΡΑΔΟΞΟΝ",   u8"παραδοξον",   "PARADOXON" },
    { u8"ΠΑΘΟΣ",       u8"παθος",       "PATHOS (Passio/Affectus)" },
    { u8"ΠΛΑΝΗΤΗΣ",    u8"πλανητης",    "PLANETES (Stella Errans)" },
    { u8"ΠΝΕΥΜΑ",      u8"πνευμα",      "PNEUMA (Spiritus/Anima)" },
    { u8"ΠΟΙΗΣΙΣ",     u8"ποιησις",     "POESIS" },
    { u8"ΠΟΛΕΜΟΣ",     u8"πολεμος",     "POLEMOS (Bellum)" },
    { u8"ΠΟΛΙΣ",       u8"πολις",       "POLIS (Civitas)" },
    { u8"ΠΡΑΞΙΣ",      u8"πραξις",      "PRAXIS (Actio)" },
    { u8"ΠΡΟΒΛΗΜΑ",    u8"προβλημα",    "PROBLEMA (Quaestio)" },
    { u8"ΠΥΡΑΜΙΣ",     u8"πυραμις",     "PYRAMIS" },
    { u8"ΠΥΡ",         u8"πυρ",         "PYR (Ignis)" },

    // --- R, S, T, Ph, Ch, Ps, O ---
    { u8"ΡΗΤΟΡΙΚΗ",    u8"ρητορικη",    "RHETORICA" },
    { u8"ΡΥΘΜΟΣ",      u8"ρυθμος",      "RHYTHMOS (Numerus)" },
    { u8"ΣΕΙΣΜΟΣ",     u8"σεισμος",     "SEISMOS (Terrae Motus)" },
    { u8"ΣΚΕΛΕΤΟΣ",    u8"σκελετος",    "SKELETOS (Ossa)" },
    { u8"ΣΟΦΙΑ",       u8"σοφια",       "SOPHIA (Sapientia)" },
    { u8"ΣΡΑΤΗΓΟΣ",    u8"στρατηγος",   "STRATEGOS (Imperator)" },
    { u8"ΣΥΜΒΟΛΟΝ",    u8"συμβολον",    "SYMBOLON (Signum)" },
    { u8"ΣΥΜΠΑΘΕΙΑ",   u8"συμπαθεια",   "SYMPATHIA (Compassio)" },
    { u8"ΣΥΜΠΟΣΙΟΝ",   u8"συμποσιον",   "SYMPOSION (Convivium)" },
    { u8"ΣΥΣΤΗΜΑ",     u8"συστημα",     "SYSTEMA" },
    { u8"ΣΦΑΙΡΑ",      u8"σφαιρα",      "SPHAERA (Globus)" },
    { u8"ΣΧΟΛΕΙΟΝ",    u8"σχολειον",    "SCHOLA (Ludus)" },
    { u8"ΤΑΧΥΤΗΣ",     u8"ταχυτης",     "TACHYTES (Celeritas)" },
    { u8"ΤΕΧΝΟΛΟΓΙΑ",  u8"τεχνολογια",  "TECHNOLOGIA (Ars Technica)" },
    { u8"ΤΗΛΕΦΩΝΟΝ",   u8"τηλεφωνον",   "TELEPHONON" },
    { u8"ΤΟΠΟΣ",       u8"τοπος",       "TOPOS (Locus)" },
    { u8"ΤΡΑΓΩΔΙΑ",    u8"τραγωδια",    "TRAGOEDIA" },
    { u8"ΤΥΡΑΝΝΟΣ",    u8"τυραννος",    "TYRANNOS (Tyrannus)" },
    { u8"ΥΔΩΡ",        u8"υδωρ",        "HYDOR (Aqua)" },
    { u8"ΥΜΝΟΣ",       u8"υμνος",       "HYMNOS (Laus)" },
    { u8"ΥΠΝΟΣ",       u8"υπνος",       "HYPNOS (Somnus)" },
    { u8"ΦΑΙΝΟΜΕΝΟΝ",  u8"φαινομενον",  "PHENOMENON" },
    { u8"ΦΑΝΤΑΣΙΑ",    u8"φαντασια",    "PHANTASIA (Imaginatio)" },
    { u8"ΦΑΡΜΑΚΟΝ",    u8"φαρμακον",    "PHARMACON (Medicamen)" },
    { u8"ΦΙΛΟΣΟΦΙΑ",   u8"φιλοσοφια",   "PHILOSOPHIA (Amor Sapientiae)" },
    { u8"ΦΟΒΙΑ",       u8"φοβια",       "PHOBIA (Timor)" },
    { u8"ΦΩΣ",         u8"φως",         "PHOS (Lux)" },
    { u8"ΧΑΟΣ",        u8"χαος",        "CHAOS" },
    { u8"ΧΑΡΑΚΤΗΡ",    u8"χαρακτηρ",    "CHARACTER (Nota/Signum)" },
    { u8"ΧΟΡΟΣ",       u8"χορος",       "CHOROS (Chorus/Saltatio)" },
    { u8"ΧΡΟΝΟΣ",      u8"χρονος",      "CHRONOS (Tempus)" },
    { u8"ΨΥΧΗ",        u8"ψυχη",        "PSYCHE (Anima)" },
    { u8"ΩΚΕΑΝΟΣ",     u8"ωκεανος",     "OCEANOS (Oceanus)" }
};

// ---------------------------------------------------------
// ÉTAT DU JEU (GAME STATE)
// ---------------------------------------------------------

enum Modus {
    LITT_GR_LAT, // Lettre Grec -> Latin
    LITT_LAT_GR, // Lettre Latin -> Grec
    VERB_GR_LAT, // Mot Grec -> Latin
    VERB_LAT_GR  // Mot Latin -> Grec
};

struct LudusStatus {
    Modus modusActualis = LITT_GR_LAT;
    
    std::string quaestio;              // Question affichée
    std::vector<std::string> optiones; // Choix
    int indexResponsum = 0;            // Index correct
    bool isMinuscula = false;          // Est-ce un tour en minuscule ?
    
    int puncta = 0;
    int tentatum = 0;
    std::string nuntius;
    bool finitus = false;
    bool responsumCorrectum = false;
};

std::random_device rd;
std::mt19937 g(rd());

int GetRandomIndex(int max) {
    std::uniform_int_distribution<> distrib(0, max - 1);
    return distrib(g);
}

void PraeparareLudum(LudusStatus& status) {
    status.finitus = false;
    status.nuntius = "Eligere responsum...";
    status.optiones.clear();
    
    // 1. Décider aléatoirement si on joue en MAJUSCULES ou minuscules (50/50)
    status.isMinuscula = (GetRandomIndex(2) == 0);

    std::string reponseCorrecte;
    std::vector<std::string> tousLesChoix;

    // 2. Sélectionner Question/Réponse selon le mode et la casse (case)
    if (status.modusActualis == LITT_GR_LAT || status.modusActualis == LITT_LAT_GR) {
        // --- LETTRES ---
        int idx = GetRandomIndex(ALPHABETUM.size());
        const auto& l = ALPHABETUM[idx];
        
        // On choisit le symbole grec approprié (Maj ou Min)
        std::string symbolumGraecum = status.isMinuscula ? l.minuscula : l.majuscula;

        if (status.modusActualis == LITT_GR_LAT) {
            status.quaestio = symbolumGraecum;
            reponseCorrecte = l.trans + " (" + l.nomen + ")";
            for(const auto& item : ALPHABETUM) tousLesChoix.push_back(item.trans + " (" + item.nomen + ")");
        } else {
            status.quaestio = l.trans + " (" + l.nomen + ")";
            reponseCorrecte = symbolumGraecum;
            // Les boutons afficheront la version Maj ou Min selon le tour actuel
            for(const auto& item : ALPHABETUM) {
                tousLesChoix.push_back(status.isMinuscula ? item.minuscula : item.majuscula);
            }
        }
    } 
    else {
        // --- MOTS ---
        int idx = GetRandomIndex(VERBA.size());
        const auto& v = VERBA[idx];
        
        std::string verbumGraecum = status.isMinuscula ? v.graecum_min : v.graecum_maj;

        if (status.modusActualis == VERB_GR_LAT) {
            status.quaestio = verbumGraecum;
            reponseCorrecte = v.latinum;
            for(const auto& item : VERBA) tousLesChoix.push_back(item.latinum);
        } else {
            status.quaestio = v.latinum;
            reponseCorrecte = verbumGraecum;
            for(const auto& item : VERBA) {
                tousLesChoix.push_back(status.isMinuscula ? item.graecum_min : item.graecum_maj);
            }
        }
    }

    // 3. Mélange et remplissage des options (comme avant)
    status.optiones.push_back(reponseCorrecte);
    
    // Supprimer la bonne réponse des leurres potentiels
    tousLesChoix.erase(std::remove(tousLesChoix.begin(), tousLesChoix.end(), reponseCorrecte), tousLesChoix.end());
    
    std::shuffle(tousLesChoix.begin(), tousLesChoix.end(), g);
    for(int i=0; i<3 && i < (int)tousLesChoix.size(); i++) {
        status.optiones.push_back(tousLesChoix[i]);
    }

    std::shuffle(status.optiones.begin(), status.optiones.end(), g);

    for(int i=0; i<status.optiones.size(); i++) {
        if(status.optiones[i] == reponseCorrecte) {
            status.indexResponsum = i;
            break;
        }
    }
}

// ---------------------------------------------------------
// MAIN
// ---------------------------------------------------------

int main(int, char**) {
    if (!glfwInit()) return 1;
    const char* glsl_version = "#version 130";
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);
    
    GLFWwindow* window = glfwCreateWindow(650, 700, "Schola Graeca II", NULL, NULL);
    if (window == NULL) return 1;
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO(); (void)io;
    
    // Font setup
    ImFont* font_magna = nullptr;
    ImFont* font_normal = nullptr;
    ImVector<ImWchar> ranges;
    ImFontGlyphRangesBuilder builder;
    builder.AddRanges(io.Fonts->GetGlyphRangesDefault());
    builder.AddRanges(io.Fonts->GetGlyphRangesGreek());
    builder.BuildRanges(&ranges);
    
    font_normal = io.Fonts->AddFontFromFileTTF("font.ttf", 22.0f, NULL, ranges.Data);
    if (font_normal == nullptr) io.Fonts->AddFontDefault(); 
    else font_magna = io.Fonts->AddFontFromFileTTF("font.ttf", 90.0f, NULL, ranges.Data);

    ImGui::StyleColorsClassic();
    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init(glsl_version);

    LudusStatus status;
    PraeparareLudum(status);

    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        ImGui::SetNextWindowPos(ImVec2(0, 0));
        ImGui::SetNextWindowSize(io.DisplaySize);
        ImGui::Begin("Schola", nullptr, ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoBringToFrontOnFocus);

        float w = ImGui::GetWindowSize().x;
        float h = ImGui::GetWindowSize().y;

        // --- EN-TÊTE ---
        ImGui::Text("Schola Graeca - Editio Secunda");
        ImGui::SameLine();
        ImGui::Text("| Puncta: %d / %d", status.puncta, status.tentatum);
        
        // Indication du mode Majuscule/Minuscule
        ImGui::SameLine(w - 200);
        ImGui::TextColored(ImVec4(0.7f, 0.7f, 1.0f, 1.0f), 
            status.isMinuscula ? "Forma: minuscula" : "Forma: MAIUSCULA"); // Minuscula / Majuscula

        ImGui::Dummy(ImVec2(0, 10));
        ImGui::Text("Modus Ludi:");
        ImGui::SameLine();
        
        const char* items[] = { 
            "Littera: GR -> LAT", 
            "Littera: LAT -> GR", 
            "Verbum: GR -> LAT", 
            "Verbum: LAT -> GR" 
        };
        static int item_current = 0;
        ImGui::PushItemWidth(250);
        if (ImGui::Combo("##combo", &item_current, items, IM_ARRAYSIZE(items))) {
            status.modusActualis = (Modus)item_current;
            status.puncta = 0;
            status.tentatum = 0;
            PraeparareLudum(status);
        }
        ImGui::PopItemWidth();
        ImGui::Separator();

        // --- QUESTION ---
        ImGui::Dummy(ImVec2(0.0f, h * 0.12f));

        if (font_magna) ImGui::PushFont(font_magna);
        float textW = ImGui::CalcTextSize(status.quaestio.c_str()).x;
        ImGui::SetCursorPosX((w - textW) * 0.5f);
        ImGui::Text("%s", status.quaestio.c_str());
        if (font_magna) ImGui::PopFont();

        // --- INSTRUCTION ---
        ImGui::Dummy(ImVec2(0.0f, h * 0.05f));
        const char* instruction = "Elige responsum verum:";
        float instW = ImGui::CalcTextSize(instruction).x;
        ImGui::SetCursorPosX((w - instW) * 0.5f);
        ImGui::Text("%s", instruction);

        // --- RÉPONSES ---
        ImGui::Dummy(ImVec2(0.0f, 30.0f));
        
        if (!status.finitus) {
            for (int i = 0; i < status.optiones.size(); i++) {
                float btnW = w * 0.5f;
                ImGui::SetCursorPosX((w - btnW) * 0.5f);
                
                if (ImGui::Button(status.optiones[i].c_str(), ImVec2(btnW, 50))) {
                    status.tentatum++;
                    status.finitus = true;
                    if (i == status.indexResponsum) {
                        status.puncta++;
                        status.nuntius = "Recte!";
                        status.responsumCorrectum = true;
                    } else {
                        status.nuntius = "Male! Rectum erat: " + status.optiones[status.indexResponsum];
                        status.responsumCorrectum = false;
                    }
                }
                ImGui::Dummy(ImVec2(0, 10));
            }
        } else {
            // --- FEEDBACK ---
            float msgW = ImGui::CalcTextSize(status.nuntius.c_str()).x;
            ImGui::SetCursorPosX((w - msgW) * 0.5f);
            
            ImVec4 col = status.responsumCorrectum ? ImVec4(0,1,0,1) : ImVec4(1,0.3f,0.3f,1);
            ImGui::TextColored(col, "%s", status.nuntius.c_str());

            ImGui::Dummy(ImVec2(0, 20));
            float nextW = w * 0.4f;
            ImGui::SetCursorPosX((w - nextW) * 0.5f);
            if (ImGui::Button("Proxima", ImVec2(nextW, 50))) {
                PraeparareLudum(status);
            }
        }

        ImGui::End();

        ImGui::Render();
        int display_w, display_h;
        glfwGetFramebufferSize(window, &display_w, &display_h);
        glViewport(0, 0, display_w, display_h);
        glClearColor(0.25f, 0.25f, 0.30f, 1.00f);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        glfwSwapBuffers(window);
    }

    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}
