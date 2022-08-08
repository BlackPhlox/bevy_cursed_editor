# bevy_cursed_editor👻

## [bevy_codegen 🚧](/codegen/) + [bevy_codegen_editor 📝](/editor/)= 👻

## `bevy_codegen`, what is it?

bevy codegen is a library for generating bevy code and provide a serializable bevy format.
Use an API to create, save and load bevy code, ideally using a gui like [egui](https://github.com/emilk/egui) at some point or it could be integrated with [bevy_editor_pls](https://github.com/jakobhellermann/bevy_editor_pls).

## `bevy_cursed_editor`, what is it?
The binary is a cli and visual editor for `bevy_codegen`.

## Usage

Call `cargo run -- --help` for help.

### Cli

Right now the default is creating a default project.
If you want to run the build and run commands you can 
do so by running `cargo run -- default default`

### Editor

Running the editor: `cargo run -- default editor`

1. Press `Open window` in the top panel and select `Cursed Overview`
2. Here you can see the current/default bevy model, go to cargo run to run the bevy app
3. Alternativly you can go to `File` > `New Project` > And select one of the templates.
4. You can also import/export files in `File` > `Import Json`/`Export Json`

## License
Not published yet and might change name, a license has not been choose yet, but most likely going  to be MIT/Apache 2.0

## Contributions
Yes please