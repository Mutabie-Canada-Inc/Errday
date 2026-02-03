# Errday
A standalone desktop app built to visualise priority tasks for startup founders using Time Management concepts

# Overview
As a startup founder, I get slammed with tasks to do everday. Theres so many different types of ways to manage your time. Pomodorro, Time Blocking, traditional TODOs list, 4 quadrent... And, theres not a lot of clear tools that can organize my thoughts in building a business. That is, without charging you or limiting core features, forcing people to sign up. 

I want to go back to the good old days of using single-use apps that don't require cloud hosting. Hosting on the cloud is great and has many benefits. But not for what I need. 

# Documentation
To learn more, please find the rest of the documentation in the docs folder. The following figure shows the folder structure of what to expect.
```
docs/
    |
    |- about.md
    L <INSERT NAME>.md
```

# How will I make this app?
I will be using Dioxus, a Rust framework for creating cross-platform apps. Rust is a systems programming language. It combines the performance of C and C++ while being memory safe and type safe. It's a game changer for writing robust code. Rust also has an amazing compiler which shows detailed logs that mean something. And it teaches you, your errors and how to fix them. Something C ad C++ doesn't have unless you install libraries.
## Tech stack
Heres a list of everything expected to be used in this application. 
- Rust
- Dioxus

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

# Contributing
If you'd like to contribute to this project, copy the following into your terminal :)
```bash
git clone https://github.com/Mutabie-Canada-Inc/Errday.git
```

# Credit 
@ 2026 Mutabie Canada Inc