mkdir ~/code/skatromb
cd ~/code/skatromb

/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)" && \
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile eval "$(/opt/homebrew/bin/brew shellenv)" && \

git clone https://github.com/skatromb/skatromb && \

brew bundle --file=~/code/skatromb/Brewfile
