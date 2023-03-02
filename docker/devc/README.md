Build docker image
```shell
docker compose build
```

Add this block to your shell's `profile` file (`~/.zprofile `for MacBooks with zsh)
```shell
# dev tools from docker
alias _devc="
  docker compose \
    --file ~/code/skatromb/docker/devc/docker-compose.yml run \
    --rm"

alias devc="_devc devc"

alias aws="_devc --entrypoint aws devc"

alias gcloud="_devc --entrypoint gcloud devc"

alias chamber="_devc --entrypoint chamber devc"
```
