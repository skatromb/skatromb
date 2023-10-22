# Idea

Here you'll find developer's container with cli tools like `glcoud` or `aws`.
The idea is to have these tools in a separate container instead of installing
all of them to your clean lovely OS,
while having pretty much the same experience.

After installation, you can use `gcloud` or `aws` commands as it was before,
except of autocompletion for their specifi commands.
If you want to have autocomplete, you can run `devc` command, which will put
you under the container's bash terminal with all autocompletes working.

Only one drawback is with browser authorization of `glcoud auth login`
and `aws sso login` — they won't open your browser automatically,
because they hook container's OS.

## Usage

To be able to use them you should:

1. Ensure you have all env-variables, mentioned in `docker-compose.yml` file.
  You can either have them in your terminal's `.*profile` file
  or put them here in `.env` file

2. Build docker image

    ```shell
    docker compose build
    ```

3. Add this block to your shell's `.*profile` file
  (`~/.zprofile` for MacBooks with zsh)

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
