# sfh-mirror

**A small API and file host for [Song File Hub](https://songfilehub.com), a website for Geometry Dash custom songs, intended to be used by mods like [Jukebox](https://github.com/Fleeym/jukebox)**

> [!note]
> The public instance is running at https://sfh.rooot.gay. <br>
> API docs can be found [here](https://github.com/RoootTheFox/sfh-mirror/wiki/Api-Docs).

## Setup
> [!caution]
> Please **do not** self host this unless you *really* need to.
> The initial sync downloads about ~8GB at the time of writing this, which can cause issues and downtime for SFH.
> **This is the whole reason this project exists.**
> Thank you for understanding.

### Requirements
- The Rust toolchain
- `sqlx`: install using `cargo install sqlx-cli`

### Setup
Before you can build the project, you have to create the database. *You only need to do this **once***:
- `sqlx db create`
- `sqlx mig run`

Then, copy `.env.example` to `.env` and adjust the values to your needs.
> [!important]
> This server **does NOT** host song files, it just downloads them to the `songs` directory.
> You'll have to set up a seperate webserver that serves the contents of that directory.
> I am using an nginx `server` for that, however the setup should be similar on other server softwares too.
> You should also **run this project behind a reverse proxy**.

### Building
- `cargo b -r` will compile the project, the target binary will be put into `./target/release/sfh-mirror`.

### Running
- execute `./target/release/sfh-mirror`. If you're running for the first time, it'll perform the initial sync, where it downloads ALL SONGS from Song File Hub.
> [!tip]
> run the program using something like [`screen`](https://www.gnu.org/software/screen/) to make it run in the background.

- once it's running, the server will listen on `0.0.0.0` port `58532`.
