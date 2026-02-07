# Contribute

Thank you for considering to contribute to this project. This document will go over the required steps to setting up your environment without seeing errors.

## Prerequisites
- Rust
- Dioxus
- Cargo
- Internet

## Cloning
```bash
git clone https://github.com/Mutabie-Canada-Inc/Errday.git
```
## Installing
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

## Serving Your App
Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```