```bash
composer-cli blueprints push microlab.toml

```
This command pushes a blueprint file named `microlab.toml` to the Composer server using the `composer-cli` command-line tool.
```bash
composer-cli blueprints push microlab-image.toml

```
This command pushes a blueprint file named `microlab-image.toml` to the Composer server using the `composer-cli` command-line tool.
```bash
composer-cli compose start-ostree --ref rhel/8/x86_64/edge microlab edge-commit

```
This command starts a new OSTree-based compose job on the Composer server using the `composer-cli` command-line tool. The `--ref` option specifies the OSTree branch to use (`rhel/8/x86_64/edge` in this case), `microlab` is the name of the blueprint to use for the compose, and `edge-commit` is the name of the commit to use.
```bash
composer-cli compose list -> get the uuid of the compose once finished

```
This command lists all the currently running and completed compose jobs on the Composer server using the `composer-cli` command-line tool, and displays their UUIDs. The user should find the UUID of the compose job they just started in order to interact with it in the subsequent commands.
```bash
composer-cli compose image <uuid> -> get the <uuid>.tar ostree archive

```
This command requests the OSTree archive for a completed compose job with the given UUID using the `composer-cli` command-line tool. The resulting file is a tar archive with a `.tar` extension.
```bash
extract ostree.tar to your webserver

```
This command directs the user to extract the OSTree archive to their webserver so that it can be accessed by other machines in order to install the image.
```bash
composer-cli compose start-ostree --ref rhel/8/x86_64/edge --url http://<your webserver>/repo/ microlab-image edge-installer

```
This command starts a new OSTree-based compose job on the Composer server using the `composer-cli` command-line tool, but this time it creates a new image called `edge-installer`. The `--ref` option specifies the OSTree branch to use (`rhel/8/x86_64/edge` in this case), `--url` specifies the URL of the OSTree repository created earlier, and `microlab-image` is the name of the blueprint to use for the compose. The resulting image is used to install the OSTree-based operating system on a target machine.

