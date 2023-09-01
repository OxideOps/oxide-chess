#!/bin/bash

# Default: run in interactive mode
DOCKER_MODE=false

# Make the server functions not change based on the path to the repo 
ENV_VAR="SERVER_FN_OVERRIDE_KEY=y"

terminate_script() {
    local message="$1"
    echo "$message" >&2
    # Exit if script is executed, return if sourced
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
}

parse_arguments() {
    while [[ "$#" -gt 0 ]]; do
        case $1 in
            -d|--docker)
                DOCKER_MODE=true
                ;;
            *)
                terminate_script "Invalid option: $1"
                ;;
        esac
        shift
    done
}

install_packages() {
    apt-get update && apt-get upgrade -y
    apt-get install -y curl libjavascriptcoregtk-4.1-dev libgtk-3-dev libsoup-3.0-dev libwebkit2gtk-4.1-dev
}

install_rust_and_cargo() {
    local cmd="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    $DOCKER_MODE && cmd+=" -s -- -y"
    eval $cmd
}

setup_nodejs() {
    curl -sL https://deb.nodesource.com/setup_20.x | bash -
    apt-get install -y nodejs
}

setup_rust_environment() {
    rustup target add wasm32-unknown-unknown
    rustup component add rustfmt
    cargo install --locked trunk
}

append_if_not_present() {
    local file="$1"
    local content="$2"
    grep -qxF "$content" "$file" || echo "$content" >> "$file"
}

setup_environment_variable() {
    local content
    case $SHELL in
    */zsh)
        content="export $ENV_VAR"
        append_if_not_present ~/.zshrc "$content"
        ;;
    */bash)
        content="export $ENV_VAR"
        append_if_not_present ~/.bashrc "$content"
        ;;
    */fish)
        content="set -gx $(echo $ENV_VAR | sed 's/=/ /')"
        append_if_not_present ~/.config/fish/config.fish "$content"
        ;;
    *)
        echo "Shell not detected. Please add the following to your shell's startup file:"
        echo "export $ENV_VAR"
        return
        ;;
    esac
    eval $content
}

update_submodules() {
    git submodule update --init --recursive
}

install_tailwind() {
    # Check for the existence of the Tailwind binary
    if [ ! -f ./node_modules/.bin/tailwindcss ]; then
        (cd client && npm install)
    fi
}

main() {
    parse_arguments "$@"
    install_packages
    install_rust_and_cargo
    install_tailwind
    setup_nodejs
    setup_rust_environment
    update_submodules
    if ! $DOCKER_MODE; then
        setup_environment_variable
        echo "Setup completed!\n\nRun 'cargo run -p client' to launch the client"
    fi
}

main "$@"
