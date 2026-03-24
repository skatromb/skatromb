bindkey "^[[A" history-beginning-search-backward
bindkey "^[[B" history-beginning-search-forward

# Brew
eval "$(/opt/homebrew/bin/brew shellenv)"
export HOMEBREW_NO_ENV_HINTS=true
source /opt/homebrew/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/completion.zsh.inc

# Other
export GIT_PRIVATE_EMAIL=1759463+skatromb@users.noreply.github.com
export PATH="$PATH:/Users/skatromb/.local/bin"  # for pyroute2
export DOCKER_HOST="unix://${HOME}/.colima/default/docker.sock"
source ~/.colima/zsh-completion
