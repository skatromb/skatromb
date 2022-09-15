/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)" && \
  echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
brew install ansible &&
ansible-galaxy install -r requirements.yml
ansible-playbook playbook.yml
