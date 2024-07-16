# meta-repo

This is a toolkit to easily manage a meta repository that includes multiple related JavaScript modules. It is written in Rust (and blazing
fast. 🚀🚀🚀).

## Terminology

- **Meta Repository**: A repository that contains multiple related repositories.
- **Package**: A JavaScript module that is published library, e.g. a component library or a domain package of a micro frontend architecture.
- **Application**: A JavaScript module that is a delivered to the end user and uses the packages e.g. the frontend application.

## Features

- Run `npm install` on multiple repositories.
- Link local dependencies automatically.
- Run `npm start` via a single command to start all applications and packages.

## Commands

| command                                 | description                                                                                      |
|-----------------------------------------|--------------------------------------------------------------------------------------------------|
| `meta-repo setup`                       | Creates a file `mata-repo.config.json` in the root directory of the meta repo.                   |
| `mete-repo init`                        | Run `git clone` for all repositories that are included.                                          |
| `meta-repo install`                     | Run `npm install` for all applications and links local packages, that are part of the meta repo. |
| `meta-repo start-app [name of the app]` | Run `npm start` for all libraries and for the application that was given.                        |

## Configuration

You need to configure the meta repository by creating a file `meta-repo.config.json` in the root directory of the meta repository.

```json
{
  "apps": [
    {
      "dir": "local directory where the app is located",
      "git": "git url of the app"
    }
  ],
  "packages": [
    {
      "dir": "local directory where the package is located",
      "git": "git url of the package"
    }
  ]
}
```