# coprocessor-operation-recording

This repository contains an example coprocessor written in [Rust](https://www.rust-lang.org/) that records operations coming into the router and records them into a CSV on a configurable interval or batch size. This repository could be modified to write to another service, such as Snowflake, for API usage analytics. The CSV written is appended to, and new ones are created after every day (UTC). 

**The code in this repository is experimental and has been provided for reference purposes only. Community feedback is welcome but this project may not be supported in the same way that repositories in the official [Apollo GraphQL GitHub organization](https://github.com/apollographql) are. If you need help you can file an issue on this repository, [contact Apollo](https://www.apollographql.com/contact-sales) to talk to an expert, or create a ticket directly in Apollo Studio.**

## Installation

Use the provided binaries within the GitHub releases or Docker image alongside following the [usage section](#usage).

## Configuration

See [`example_config.yaml`](./example_config.yaml) for a complete example of a configuration.

### Generate a configuration schema
To generate a YAML schema file you can run the binary with the command `config-schema`. This file can then be configured in your config YAML file to provide autocomplete and validation of the YAML file right in your IDE

```shell
# forces the application to print out a JSON schema for YAML validation
./coprocessor-operation-recording config-schema > config-schema.json
```
OR
```shell
docker run ghcr.io/apollosolutions/coprocessor-operation-recording:latest config-schema > config-schema.json
```

* See [example_config.yaml](./example_config.yaml) to see an example of setting it up
* We use [Red Hat's YAML extension for VSCode](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) to validate

### Configuration options

#### `listen`

This defines the listen address for the coprocessor. The default is `127.0.0.1:4000`. 

#### `interval`

The interval period, in seconds, for writing the CSV. This defaults to 5 seconds, and must be greater than 0.

#### `batch_size`

This determines the maximum batch size held in memory before writing to the CSV. When set to 0, the batch size is unlimited and only writes during the defined `interval`. 

## Usage

When running, you'll need to configure your router as such:

```yml
coprocessor:
  url: http://localhost:4000/ # the listening address defined in your configuration
  router:
    response:
      context: true 
      status_code: true
```

By default, the CSV contains: 

* Unique request ID
* Timestamp of the operation, in UTC following the RFC3339 format
* Operation name, if available
* Client name and version
* Resulting status code

This code could be modified to include other information, including custom information (e.g. a value from a header/another context value) or cost data. 

## Known Limitations

List any limitations of the project here:

- Requires specific configuration of the router to work
- Only writes to disk as of today, and only writes to the path in which the command is run
- For those using a coprocessor, you will need to either update your coprocessor with similar logic, or include your logic (in Rust) in a fork of this repository
