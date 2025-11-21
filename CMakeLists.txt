cmake_minimum_required(VERSION 3.14)
project(ScholaGraeca)

set(CMAKE_CXX_STANDARD 17)

include(FetchContent)

# 1. Télécharger GLFW
FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw
    GIT_TAG        3.3.8
)
FetchContent_MakeAvailable(glfw)

# 2. Télécharger ImGui
FetchContent_Declare(
    imgui
    GIT_REPOSITORY https://github.com/ocornut/imgui
    GIT_TAG        docking # Branche active
)
FetchContent_MakeAvailable(imgui)

# 3. Créer l'exécutable
add_executable(ScholaGraeca main.cpp)

# 4. Configurer les sources d'ImGui pour OpenGL et GLFW
target_include_directories(ScholaGraeca PRIVATE 
    ${imgui_SOURCE_DIR} 
    ${imgui_SOURCE_DIR}/backends
)

# Ajouter les fichiers d'implémentation ImGui nécessaires
target_sources(ScholaGraeca PRIVATE
    ${imgui_SOURCE_DIR}/imgui.cpp
    ${imgui_SOURCE_DIR}/imgui_draw.cpp
    ${imgui_SOURCE_DIR}/imgui_tables.cpp
    ${imgui_SOURCE_DIR}/imgui_widgets.cpp
    ${imgui_SOURCE_DIR}/imgui_demo.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_glfw.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_opengl3.cpp
)

# 5. Lier les bibliothèques
target_link_libraries(ScholaGraeca PRIVATE glfw)

# Gestion des libs système selon l'OS
if(WIN32)
    target_link_libraries(ScholaGraeca PRIVATE opengl32)
elseif(APPLE)
    target_link_libraries(ScholaGraeca PRIVATE "-framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo")
else()
    target_link_libraries(ScholaGraeca PRIVATE GL)
endif()

# Copier le fichier de police (si tu en as un) après la compilation
# file(COPY font.ttf DESTINATION ${CMAKE_BINARY_DIR})
