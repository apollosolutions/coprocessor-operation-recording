# Solutions Repo Template

Give your project a relevant name and description in this README, and rename CHANGE-ME to the correct repository name.

**The code in this repository is experimental and has been provided for reference purposes only. Community feedback is welcome but this project may not be supported in the same way that repositories in the official [Apollo GraphQL GitHub organization](https://github.com/apollographql) are. If you need help you can file an issue on this repository, [contact Apollo](https://www.apollographql.com/contact-sales) to talk to an expert, or create a ticket directly in Apollo Studio.**

## Installation

Outline the steps required to install the example.

## Configuration

See [`example_config.yaml`](./example_config.yaml) for a complete example of a configuration.

### Generate a configuration schema
To generate a YAML schema file you can run the binary with the command `config-schema`. This file can then be configured in your config YAML file to provide autocomplete and validation of the YAML file right in your IDE

```shell
# forces the application to print out a JSON schema for YAML validation
./CHANGE-ME config-schema > config-schema.json
```
OR
```shell
docker run ghcr.io/apollosolutions/CHANGE-ME:latest config-schema > config-schema.json
```

* See [example_config.yaml](./example_config.yaml) to see an example of setting it up
* We use [Red Hat's YAML extension for VSCode](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) to validate

## Usage

Provide detailed usage instructions here.

## Known Limitations

List any limitations of the project here:

- Limitation 1
- Limitation 2

## Notes

Is there anything else the user should know about this project? (e.g. assumptions, prior art, references, etc.)
