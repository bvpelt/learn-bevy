use bevy::prelude::*;

pub fn ep10() {
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
