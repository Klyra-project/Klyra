# Persist Example

An example app to show what you can do with klyra.

## How to deploy the example

To deploy the examples, check out the repository locally

```bash
$ git clone https://github.com/klyra-hq/klyra.git
```

navigate to the Persist root folder

```bash
$ cd examples/rocket/persist
```

Pick a project name that is something unique - in klyra,
projects are globally unique. Then run

```bash
$ cargo klyra project new --name=$PROJECT_NAME
$ cargo klyra deploy --name=$PROJECT_NAME
```

Once deployed you can post to the endpoint the following values:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"date":"2020-12-22", "temp_high":5, "temp_low":5, "precipitation": 5}' {$PROJECT_NAME}.klyraapp.rs
```

The json data will then persist within Klyra it can be queried with the following curl request

```bash
curl {$PROJECT_NAME}.klyraapp.rs/2020-12-22
```
