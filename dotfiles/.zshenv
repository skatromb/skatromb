# Terminal
bindkey "^[[A" history-beginning-search-backward
bindkey "^[[B" history-beginning-search-forward

# Brew
eval "$(/opt/homebrew/bin/brew shellenv)"
export HOMEBREW_NO_ENV_HINTS=true

# Rust
. "$HOME/.cargo/env"

# Go
export GOPATH="$HOME/.local/share/go"
export GOBIN="$HOME/.local/bin"

# Other
source /opt/homebrew/share/google-cloud-sdk/completion.zsh.inc
source ~/.colima/zsh-completion
export GIT_PRIVATE_EMAIL=1759463+skatromb@users.noreply.github.com
export DOCKER_HOST="unix://${HOME}/.colima/default/docker.sock"
