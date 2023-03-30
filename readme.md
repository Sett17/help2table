# Introduction

`help2table` is a tool that uses OpenAI AI models to generate a markdown table of arguments from the help message of a command. It is designed for developers and software maintainers who need to generate documentation for their pipelines.

With `help2table`, you can easily generate a markdown table that can be used to document your pipeline and ensure that your documentation is always up to date. We created `help2table` to make it easier for you to generate accurate documentation, without the need for manual labor.

# Installation

To install `help2table`, you can use the following command:


```bash
$ cargo install help2table
```

This will install `help2table` via Cargo, the Rust package manager. There are no system requirements or dependencies for installation, so you should be able to install `help2table` without any issues.

# Usage

The basic usage of `help2table` is as follows:

`Usage: help2table [OPTIONS] <COMMAND>`

To generate a markdown table from the help message of a command, simply run `help2table` followed by the command that returns the help message.

Here are some important command line options and arguments that users should be aware of:

| Short | Long        | Description                       | Default     |
| ----- | ----------- | --------------------------------- | ----------- |
|       | COMMAND     | Command that returns help message |             |
| -m    | --model     | Model to use                      | gpt35-turbo |
| -p    | --pipable   | Print only the table output       |             |
| -c    | --clipboard | Put the table output in clipboard |             |
| -h    | --help      | Print help                        |             |
| -V    | --version   | Print version                     |             |

# License

`help2table2` is released under the MIT License. Please see the [LICENSE](/LICENSE) file for more information.