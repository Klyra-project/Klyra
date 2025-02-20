# Examples

Some example apps to show what you can do with klyra.

## How to deploy the examples

To deploy the examples, check out the repository locally

```bash
$ git clone https://github.com/getsynth/klyra.git
```

navigate to an example root folder

```bash
$ cd examples/rocket/hello-world
```

open up the `Klyra.toml` file and change the project name to something 
unique - in klyra, projects are globally unique. Then run

```bash
$ cargo klyra deploy
```