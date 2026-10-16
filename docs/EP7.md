
# EP7

Based on https://www.youtube.com/watch?v=gy2G63SA-W8&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=7

Learning targets
- refactor the project into rust modules, simple refactor
- use rust modules and bevy plugins, plugin modules

## Simple refactor

Split the current implementation in modules

```text
src\
    main.rs
    components.rs
    events.rs
    resources.rs
    systems.rs
```

Modules is a rust concept, used to split functionallity into logical units.

For a project of this size this would do. [github](https://github.com/bvpelt/learn-bevy/tree/simple-refactor)

## Plugin Modules

To have an idea of using plugins use the following structure

```text
src\
    enemy\
        mod.rs
        components.rs
        resources.rs
        systems.rs
    player\
        mod.rs
        components.rs
        systems.rs
    score\
        mod.rs
        resources.rs
        systems.rs
    star\
        mod.rs
        components.rs
        resources.rs
        systems.rs
    main.rs
    event.rs
```