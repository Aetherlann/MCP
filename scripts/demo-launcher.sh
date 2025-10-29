#!/bin/bash
# Interactive demo launcher for Hyper Terminal
# Shows a menu of all available demos and runs the selected one

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Function to print header
print_header() {
    echo -e "${CYAN}${BOLD}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║          HYPER TERMINAL - DEMO LAUNCHER                    ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Function to print demo info
print_demo_info() {
    local num=$1
    local name=$2
    local desc=$3
    echo -e "${GREEN}${BOLD}[$num]${NC} ${YELLOW}$name${NC}"
    echo -e "    ${desc}"
    echo
}

# Main menu
show_menu() {
    clear
    print_header

    echo -e "${BOLD}Available Demos:${NC}"
    echo

    # Gallery demos
    echo -e "${CYAN}${BOLD}Gallery System Demos (NEW!)${NC}"
    print_demo_info "1" "Grid Gallery" \
        "Simple 3-image grid layout with metadata"

    print_demo_info "2" "Comparison Gallery" \
        "Before/after side-by-side comparison"

    print_demo_info "3" "All Gallery Modes" \
        "Comprehensive demo of all 5 gallery modes"

    # Media demos
    echo -e "${CYAN}${BOLD}Media Display Demos${NC}"
    print_demo_info "4" "Basic Images" \
        "Kitty and iTerm2 protocol image display"

    print_demo_info "5" "Multiple Images" \
        "Display several images simultaneously"

    print_demo_info "6" "Large Images" \
        "Test with high-resolution images"

    # Terminal demos
    echo -e "${CYAN}${BOLD}Terminal Features${NC}"
    print_demo_info "7" "Colors and Attributes" \
        "256 colors, RGB, bold, italic, underline"

    print_demo_info "8" "Unicode and Emoji" \
        "Box drawing, emoji, international text"

    print_demo_info "9" "All Features" \
        "Comprehensive demo of ALL terminal features"

    # Special options
    echo -e "${CYAN}${BOLD}Special${NC}"
    print_demo_info "A" "Run All Demos" \
        "Execute all demos in sequence"

    print_demo_info "0" "Exit" \
        "Return to terminal"

    echo
}

# Function to run a demo with header
run_demo() {
    local script=$1
    local name=$2

    clear
    echo -e "${CYAN}${BOLD}"
    echo "═══════════════════════════════════════════════════════════"
    echo "  Running: $name"
    echo "═══════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo

    if [ -f "$script" ]; then
        bash "$script"
        echo
        echo -e "${GREEN}${BOLD}Demo complete!${NC}"
    else
        echo -e "${RED}${BOLD}Error: Demo script not found: $script${NC}"
    fi

    echo
    echo -e "${YELLOW}Press Enter to return to menu...${NC}"
    read
}

# Main loop
while true; do
    show_menu

    echo -ne "${BOLD}Select demo [1-9, A, or 0 to exit]: ${NC}"
    read choice

    case $choice in
        1)
            run_demo "demo-gallery-grid.sh" "Grid Gallery Demo"
            ;;
        2)
            run_demo "demo-gallery-comparison.sh" "Comparison Gallery Demo"
            ;;
        3)
            run_demo "demo-gallery-all-modes.sh" "All Gallery Modes Demo"
            ;;
        4)
            run_demo "demo-images.sh" "Basic Images Demo"
            ;;
        5)
            run_demo "demo-multiple-images.sh" "Multiple Images Demo"
            ;;
        6)
            run_demo "demo-large-image.sh" "Large Images Demo"
            ;;
        7)
            run_demo "demo-colors.sh" "Colors and Attributes Demo"
            ;;
        8)
            run_demo "demo-unicode.sh" "Unicode and Emoji Demo"
            ;;
        9)
            run_demo "run-all-demos.sh" "All Features Demo"
            ;;
        [Aa])
            clear
            echo -e "${CYAN}${BOLD}Running all demos in sequence...${NC}"
            echo

            for script in demo-*.sh; do
                if [ "$script" != "demo-launcher.sh" ]; then
                    echo -e "${YELLOW}▶ $script${NC}"
                    bash "$script"
                    echo
                    sleep 2
                fi
            done

            echo -e "${GREEN}${BOLD}All demos complete!${NC}"
            echo -e "${YELLOW}Press Enter to return to menu...${NC}"
            read
            ;;
        0)
            clear
            echo -e "${GREEN}${BOLD}Thanks for trying Hyper Terminal!${NC}"
            echo
            exit 0
            ;;
        *)
            echo -e "${RED}Invalid choice. Press Enter to try again...${NC}"
            read
            ;;
    esac
done
