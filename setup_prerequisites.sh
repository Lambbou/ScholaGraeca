#!/usr/bin/env bash
set -e

echo "================================================================="
echo "   ScholaGraeca - Installatio Praerequisitorum (Bevy & Rust)     "
echo "================================================================="
echo ""
echo "Hic fasciculus necessarias bibliothecas et instrumenta ad ludum"
echo "ScholaGraeca compilandum sub Linux (Ubuntu/Debian) installat."
echo ""

# Vérification des privilèges
if [ "$EUID" -ne 0 ]; then
  echo "Quaeso, hoc scriptum cum 'sudo' exsequere:"
  echo "  sudo bash setup_prerequisites.sh"
  exit 1
fi

echo "[1/3] Renovantur fasciculi apt (apt-get update)..."
apt-get update

echo "[2/3] Installantur bibliothecae graphicae, soni et compilatoris..."
apt-get install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libx11-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libgl1-mesa-dev \
    clang \
    lld

echo "[3/3] Verificatio instrumentorum..."
echo -n "  pkg-config: " && pkg-config --version
echo -n "  clang:      " && clang --version | head -n 1
echo -n "  lld:        " && lld --version | head -n 1

echo ""
echo "================================================================="
echo "  Omnia praerequisita feliciter installata sunt!                "
echo "  Nunc inceptum Rust ScholaGraeca compilari potest.             "
echo "================================================================="
