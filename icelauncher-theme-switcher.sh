#!/usr/bin/env bash

# ╭─────────────────────────────────────────────────────────────────────────╮
# │   ICELAUNCHER THEME SWITCHER                                            │
# │   Lists themes in ./themes, lets you pick one,                          │
# │   and installs its config.ron to ~/.config/icelauncher/                 │
# │                                                                         │
# │   Flags:                                                                │
# │     --no-exit      -n          Loop back to theme list after install.   │
# │     --force        -f          Skip confirmation prompts.               │
# │     --cycle        -c          Cycle through all themes one by one.     │
# │     --select       -s  <name>  Directly install theme by name.          │
# │     --help         -h          Show this help message and exit.         │
# ╰─────────────────────────────────────────────────────────────────────────╯

# ── Flags ──────────────────────────────────────────────────────────────────
NO_EXIT=false
FORCE=false
CYCLE=false
SELECT=""       # theme name to install directly

# ── Help ───────────────────────────────────────────────────────────────────
print_help()
{
    echo
    echo -e "${CYAN}${BOLD}  icelauncher-theme-switcher${RESET}"
    echo
    echo -e "  ${BWHITE}Usage:${RESET}"
    echo -e "    ${DIM}./icelauncher-theme-switcher.sh [flags]${RESET}"
    echo
    echo -e "  ${BWHITE}Flags:${RESET}"
    echo -e "    ${CYAN}-n${RESET}, ${CYAN}--no-exit${RESET}              Loop back to the theme list after installing."
    echo -e "    ${CYAN}-f${RESET}, ${CYAN}--force${RESET}                Skip all confirmation prompts."
    echo -e "    ${CYAN}-c${RESET}, ${CYAN}--cycle${RESET}                Cycle through every theme one by one."
    echo -e "    ${CYAN}-s${RESET}, ${CYAN}--select${RESET}     ${DIM}<name>${RESET}     Directly install a theme by name."
    echo -e "    ${CYAN}-h${RESET}, ${CYAN}--help${RESET}                 Show this help message and exit."
    echo
    echo -e "  ${BWHITE}Examples:${RESET}"
    echo -e "    ${DIM}./icelauncher-theme-switcher.sh${RESET}"
    echo -e "    ${DIM}./icelauncher-theme-switcher.sh --cycle${RESET}"
    echo -e "    ${DIM}./icelauncher-theme-switcher.sh --force --no-exit${RESET}"
    echo -e "    ${DIM}./icelauncher-theme-switcher.sh --select dracula${RESET}"
    echo
}

# ── Argument parsing ────────────────────────────────────────────────────────
i=1
while [[ $i -le $# ]]; do
    arg="${!i}"
    case "$arg" in
        --help|-h)      print_help; exit 0 ;;
        --no-exit|-n)   NO_EXIT=true ;;
        --force|-f)     FORCE=true   ;;
        --cycle|-c)     CYCLE=true   ;;
        --select|-s)
            i=$(( i + 1 ))
            [[ $i -gt $# ]] && { echo -e "  ${RED}${BOLD}✗${RESET}  --select/-s requires a theme name." >&2; exit 1; }
            SELECT="${!i}"
            ;;
    esac
    i=$(( i + 1 ))
done

# ── Paths ───────────────────────────────────────────────────────────────────
THEMES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/themes"
LAUNCHER_DIR="$HOME/.config/icelauncher"
LAUNCHER_CONFIG="$LAUNCHER_DIR/config.ron"

# ── Colors ──────────────────────────────────────────────────────────────────
RESET='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
WHITE='\033[0;37m'
BWHITE='\033[1;37m'

# ── Helpers ──────────────────────────────────────────────────────────────────
print_header() {
    echo
    echo -e "${CYAN}${BOLD}  ╭──────────────────────────────────────────╮${RESET}"
    echo -e "${CYAN}${BOLD}  │       ICELAUNCHER THEME SWITCHER         │${RESET}"
    echo -e "${CYAN}${BOLD}  ╰──────────────────────────────────────────╯${RESET}"
    echo
}

print_success() { echo -e "  ${GREEN}${BOLD}✓${RESET}  $1"; }
print_error()   { echo -e "  ${RED}${BOLD}✗${RESET}  $1"; }
print_info()    { echo -e "  ${CYAN}→${RESET}  $1"; }
print_warn()    { echo -e "  ${YELLOW}!${RESET}  $1"; }

divider() { echo -e "  ${DIM}──────────────────────────────────────────${RESET}"; }

# ── Main loop (repeated when --no-exit or --cycle is set) ──────────────────
CYCLE_INDEX=0
while true; do

# ── Sanity checks ───────────────────────────────────────────────────────────
if [[ ! -d "$THEMES_DIR" ]]; then
    print_header
    print_error "Themes folder not found: ${BOLD}$THEMES_DIR${RESET}"
    print_info  "Create a folder called ${BOLD}themes${RESET} next to this script"
    print_info  "and place theme subdirectories inside it, each containing a ${BOLD}config.ron${RESET}"
    echo
    exit 1
fi

# ── Collect themes ───────────────────────────────────────────────────────────
mapfile -t THEMES < <(
    for dir in "$THEMES_DIR"/*/; do
        [[ -f "$dir/config.ron" ]] && basename "$dir"
    done | sort
)

if [[ ${#THEMES[@]} -eq 0 ]]; then
    print_header
    print_error "No themes found in ${BOLD}$THEMES_DIR${RESET}"
    print_info  "Each theme must be a subdirectory containing a ${BOLD}config.ron${RESET} file"
    echo
    exit 1
fi

# ── Cycle mode: auto-select next theme ──────────────────────────────────────
if [[ "$CYCLE" == true ]]; then
    if (( CYCLE_INDEX >= ${#THEMES[@]} )); then
        echo
        print_info "All ${#THEMES[@]} themes have been cycled through."
        echo
        exit 0
    fi
    CHOSEN_THEME="${THEMES[$CYCLE_INDEX]}"
    CHOSEN_CONFIG="$THEMES_DIR/$CHOSEN_THEME/config.ron"
    CYCLE_INDEX=$(( CYCLE_INDEX + 1 ))

    print_header
    echo -e "  ${DIM}Theme ${CYCLE_INDEX} of ${#THEMES[@]}${RESET}"
    echo
    divider
    echo
    echo -e "  ${BWHITE}Selected:${RESET}  ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    echo

else

# ── Direct select mode: install by name ─────────────────────────────────────
if [[ -n "$SELECT" ]]; then
    CHOSEN_CONFIG="$THEMES_DIR/$SELECT/config.ron"
    if [[ ! -f "$CHOSEN_CONFIG" ]]; then
        print_header
        print_error "Theme '${BOLD}${SELECT}${RESET}' not found in ${BOLD}$THEMES_DIR${RESET}"
        echo
        print_info "Available themes:"
        echo
        for t in "${THEMES[@]}"; do
            echo -e "    ${DIM}·${RESET}  ${WHITE}${t}${RESET}"
        done
        echo
        exit 1
    fi
    CHOSEN_THEME="$SELECT"
    print_header
    divider
    echo
    echo -e "  ${BWHITE}Selected:${RESET}  ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    echo

else

# ── Display theme list ───────────────────────────────────────────────────────
print_header

echo -e "  ${BWHITE}Available themes:${RESET}"
echo
for i in "${!THEMES[@]}"; do
    num=$(printf "%2d" $((i + 1)))
    echo -e "  ${DIM}${num}.${RESET}  ${WHITE}${THEMES[$i]}${RESET}"
done

echo
divider
echo

# ── Prompt selection ─────────────────────────────────────────────────────────
while true; do
    echo -ne "  ${BWHITE}Select a theme ${DIM}[1-${#THEMES[@]}]${RESET}${BWHITE} or ${DIM}[q]${RESET}${BWHITE} to quit:${RESET} "
    read -r selection

    if [[ "$selection" =~ ^[qQ]$ ]]; then
        echo
        print_info "Aborted. No changes made."
        echo
        exit 0
    fi

    if [[ "$selection" =~ ^[0-9]+$ ]] &&
       (( selection >= 1 && selection <= ${#THEMES[@]} )); then
        CHOSEN_THEME="${THEMES[$((selection - 1))]}"
        CHOSEN_CONFIG="$THEMES_DIR/$CHOSEN_THEME/config.ron"
        break
    fi

    print_warn "Invalid choice. Enter a number between 1 and ${#THEMES[@]}, or q to quit."
done

fi  # end of select mode
fi  # end of cycle/manual selection

echo
divider
echo

# ── Ensure icelauncher config dir exists ─────────────────────────────────────
mkdir -p "$LAUNCHER_DIR"

# ── Handle existing config ───────────────────────────────────────────────────
if [[ -f "$LAUNCHER_CONFIG" ]]; then
    if [[ "$FORCE" == true || "$CYCLE" == true ]]; then
        print_info "Overwriting existing config."
    else
        print_warn "A config already exists at ${BOLD}$LAUNCHER_CONFIG${RESET}"
        echo
        echo -e "  ${BWHITE}What would you like to do?${RESET}"
        echo
        echo -e "  ${DIM}1.${RESET}  ${WHITE}Create a backup and install the new theme${RESET}"
        echo -e "  ${DIM}2.${RESET}  ${WHITE}Overwrite the current config${RESET}"
        echo -e "  ${DIM}q.${RESET}  ${WHITE}Cancel${RESET}"
        echo

        while true; do
            echo -ne "  ${BWHITE}Choose an option ${DIM}[1/2/q]:${RESET} "
            read -r conflict_choice

            case "$conflict_choice" in

                1)
                    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
                    BACKUP_PATH="$LAUNCHER_DIR/config.ron.backup_$TIMESTAMP"
                    if cp "$LAUNCHER_CONFIG" "$BACKUP_PATH"; then
                        echo
                        print_success "Backup created: ${DIM}$BACKUP_PATH${RESET}"
                    else
                        echo
                        print_error "Failed to create backup. Aborting."
                        echo
                        exit 1
                    fi
                    break
                    ;;

                2)
                    echo
                    print_warn "${BOLD}${RED}This will permanently overwrite your current config.${RESET}"
                    echo
                    echo -ne "  ${BWHITE}Are you sure you want to overwrite? ${DIM}[yes/no]:${RESET} "
                    read -r confirm

                    if [[ "$confirm" == "yes" ]]; then
                        echo
                        print_info "Proceeding with overwrite..."
                        break
                    else
                        echo
                        print_info "Overwrite cancelled. No changes made."
                        echo
                        exit 0
                    fi
                    ;;

                [qQ])
                    echo
                    print_info "Cancelled. No changes made."
                    echo
                    exit 0
                    ;;

                *)
                    print_warn "Invalid option. Enter 1, 2, or q."
                    ;;
            esac
        done
    fi
fi

# ── Copy the chosen config ────────────────────────────────────────────────────
echo
if cp "$CHOSEN_CONFIG" "$LAUNCHER_CONFIG"; then

    divider
    echo
    print_success "${BOLD}Theme installed successfully!${RESET}"
    echo
    print_info "Theme:   ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    print_info "Config:  ${DIM}$LAUNCHER_CONFIG${RESET}"
    echo
    divider
    echo
    echo -e "  ${DIM}Relaunch icelauncher to apply the new theme.${RESET}"
    echo

    # ── Loop back or exit ─────────────────────────────────────────────────────
    if [[ "$CYCLE" == true ]]; then
        if (( CYCLE_INDEX < ${#THEMES[@]} )); then
            echo
            echo -ne "  ${BWHITE}Try next theme (${THEMES[$CYCLE_INDEX]})? ${DIM}[y/n/q]:${RESET} "
            read -r cycle_ans
            case "${cycle_ans,,}" in
                y|"") echo; continue ;;
                q)    echo; print_info "Stopped cycling."; echo; exit 0 ;;
                *)
                    echo
                    print_info "Keeping current theme. Goodbye."
                    echo
                    exit 0
                    ;;
            esac
        fi
    elif [[ "$NO_EXIT" == true ]]; then
        echo -ne "  ${BWHITE}Switch to another theme? ${DIM}[y/n]:${RESET} "
        read -r again
        if [[ "$again" =~ ^[yY]$ ]]; then
            echo
            continue
        else
            echo
            print_info "Goodbye."
            echo
            exit 0
        fi
    fi

else
    echo
    print_error "Failed to copy config. Check permissions for ${BOLD}$LAUNCHER_DIR${RESET}"
    echo
    exit 1
fi

break
done
