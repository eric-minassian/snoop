# Snoop

Simple, and bear-bones mono-repo build tool.

## Instructions

1. Create a `snoop.json` file in the root of your project.
2. Add the following configuration:

```json
{
  "packages": [
    {
      "name": "package-name",
      "root": "path/to/package"
    }
  ]
}
```

3. Create a `snoop.json` file in the root of each package.
4. Add the following configuration:

```json
{
  "commands": [
    {
      "name": "command-name",
      "command": "command"
    }
  ]
}
```

5. Run `snoop` in the root of your project. `snoop package-name command-name` to run a command in a specific package.
