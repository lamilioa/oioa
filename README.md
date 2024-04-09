# oioa 💬
oioa is a simple rust-based api that can serve the lami lioa dictionary.

## usage
it requires the lami lioa dictionary in the yaml format as maintained in the [lami](https://github.com/lamilioa/lami) repo.
by default, it looks for a file called `dictionary.yaml` in the same folder that the application is being run in but this can be overridden with the `--filename` command-line argument.

the port that the api runs on can be set with the `ROCKET_PORT` environment variable.