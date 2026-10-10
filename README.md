# Bevy tutorial

Based on https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd 

## Step 1

The simplest bevy app

```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```
When this application is started ```cargo run``` it terminates immeadiately, since it does nothing.

## Step 2

Add functionality

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, hello_world).run();
}

pub fn hello_world() {
    println!("Hello, world!");
}
```
When this application is started it print 'Hello, world!' and exits

## Step 3

Add a component

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, setup).run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}
```
When this application is started ```cargo run``` it terminates immeadiately, since it does nothing.


https://youtu.be/TQt-v_bFdao?si=HuPLi_gimJtk2O-h&t=775