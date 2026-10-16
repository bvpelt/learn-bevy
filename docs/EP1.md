
# EP1

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

## Step 4 

To make Step 3 working, add a query

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}
#[derive(Component)]
pub struct Person {
    pub name: String,
}
```

Running
```bash
$ cargo run
   Compiling learn-bevy v0.1.0 (/home/bvpelt/Develop/learn-bevy)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
     Running `target/debug/learn-bevy`
Name: Alex
```

## Step 5
Add more persons

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn(Person {
        name: "Charlie".to_string(),
    });
    commands.spawn(Person {
        name: "David".to_string(),
    });
    commands.spawn(Person {
        name: "Ellen".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}
#[derive(Component)]
pub struct Person {
    pub name: String,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/learn-bevy`
Name: Alex
Name: Bob
Name: Charlie
Name: David
Name: Ellen
```

## Step 6
Add employeed status to a number of persons

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .add_systems(Update, people_with_jobs)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    // This entity does not have an Employed component, so it will not be printed by the print_with_jobssystem.
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}
#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/learn-bevy`
Alex has a job
Charlie has a job
David has a job
Ellen has a job
Name: Bob
Name: Alex
Name: Charlie
Name: David
Name: Ellen
```

## Step 7

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update, people_without_jobs)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    // This entity does not have an Employed component, so it will not be printed by the print_with_jobssystem.
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    println!("All people:");
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    println!("People with jobs:");
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    println!("People without jobs:");
    for person in person_query.iter() {
        println!("{} does not have a job", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/learn-bevy`
People with jobs:
Alex has a job
Charlie has a job
David has a job
Ellen has a job
People without jobs:
Bob does not have a job
All people:
Name: Bob
Name: Alex
Name: Charlie
Name: David
Name: Ellen
```

## Step 8
Combined query with people and employed status. Running systems in order.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                print_names,
                people_with_jobs,
                people_without_jobs,
                people_does_job,
            )
                .chain(),
        )
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    // This entity does not have an Employed component, so it will not be printed by the print_with_jobssystem.
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    println!("All people:");
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    println!("People with jobs:");
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    println!("People without jobs:");
    for person in person_query.iter() {
        println!("{} does not have a job", person.name);
    }
}

pub fn people_does_job(person_query: Query<(&Person, &Employed)>) {
    println!("People and their jobs:");
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{0} is a {1}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/learn-bevy`
All people:
Name: Bob
Name: Alex
Name: Charlie
Name: David
Name: Ellen
People with jobs:
Alex has a job
Charlie has a job
David has a job
Ellen has a job
People without jobs:
Bob does not have a job
People and their jobs:
Alex is a Doctor
Charlie is a FireFighter
David is a Lawyer
Ellen is a FireFighter
```

## Step 9
Adding plugins

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                print_names,
                people_with_jobs,
                people_without_jobs,
                people_does_job,
            )
                .chain(),
        )
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    // This entity does not have an Employed component, so it will not be printed by the print_with_jobssystem.
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    println!("All people:");
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    println!("People with jobs:");
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    println!("People without jobs:");
    for person in person_query.iter() {
        println!("{} does not have a job", person.name);
    }
}

pub fn people_does_job(person_query: Query<(&Person, &Employed)>) {
    println!("People and their jobs:");
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{0} is a {1}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/learn-bevy`
2026-10-14T06:35:52.655834Z  INFO bevy_render::renderer: AdapterInfo { name: "Intel(R) Graphics (MTL)", vendor: 32902, device: 32069, device_type: IntegratedGpu, device_pci_bus_id: "0000:00:02.0", driver: "Intel open-source Mesa driver", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4", backend: Vulkan, subgroup_min_size: 8, subgroup_max_size: 32, transient_saves_memory: false }
2026-10-14T06:35:53.054159Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-10-14T06:35:53.054294Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-10-14T06:35:53.056553Z  INFO bevy_winit::system: Creating new window learn-bevy (65v0)
2026-10-14T06:35:53.058507Z  INFO winit::platform_impl::linux::x11::window: Guessed window scale factor: 1
All people:
Name: Bob
Name: Alex
Name: Charlie
Name: David
Name: Ellen
People with jobs:
Alex has a job
Charlie has a job
David has a job
Ellen has a job
People without jobs:
Bob does not have a job
People and their jobs:
Alex is a Doctor
Charlie is a FireFighter
David is a Lawyer
Ellen is a FireFighter
...
```

A window opens and each add_system function is called
- setup is only called once at startup
- print_names, people_with_jobs, people_without_jobs,people_does_job are called at each redraw of the opened window.


## Step 10
Create a user plugin for all the people functionallity

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                print_names,
                people_with_jobs,
                people_without_jobs,
                people_does_job,
            )
                .chain(),
        );
    }
}
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    // This entity does not have an Employed component, so it will not be printed by the print_with_jobssystem.
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    println!("All people:");
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    println!("People with jobs:");
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    println!("People without jobs:");
    for person in person_query.iter() {
        println!("{} does not have a job", person.name);
    }
}

pub fn people_does_job(person_query: Query<(&Person, &Employed)>) {
    println!("People and their jobs:");
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{0} is a {1}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
```

Running
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/learn-bevy`
2026-10-14T06:49:04.992970Z  INFO bevy_render::renderer: AdapterInfo { name: "Intel(R) Graphics (MTL)", vendor: 32902, device: 32069, device_type: IntegratedGpu, device_pci_bus_id: "0000:00:02.0", driver: "Intel open-source Mesa driver", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4", backend: Vulkan, subgroup_min_size: 8, subgroup_max_size: 32, transient_saves_memory: false }
2026-10-14T06:49:05.394551Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-10-14T06:49:05.394690Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-10-14T06:49:05.396896Z  INFO bevy_winit::system: Creating new window learn-bevy (65v0)
2026-10-14T06:49:05.398744Z  INFO winit::platform_impl::linux::x11::window: Guessed window scale factor: 1
All people:
Name: Bob
Name: Alex
Name: Charlie
Name: David
Name: Ellen
People with jobs:
Alex has a job
Charlie has a job
David has a job
Ellen has a job
People without jobs:
Bob does not have a job
People and their jobs:
Alex is a Doctor
Charlie is a FireFighter
David is a Lawyer
Ellen is a FireFighter
```
The output is simular to [Step 10](#step-10)

