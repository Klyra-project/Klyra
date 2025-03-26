# Examples

Some example apps to show what you can do with klyra.

## How to deploy the examples

To deploy the examples, check out the repository locally

```bash
$ git clone https://github.com/klyra-hq/klyra.git
```

navigate to an example root folder

```bash
$ cd examples/axum/hello-world
```

Pick a project name that is something unique - in klyra,
projects are globally unique. Then run

```bash
$ cargo klyra project new --name=$PROJECT_NAME
$ cargo klyra deploy --name=$PROJECT_NAME
```
