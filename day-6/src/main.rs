use std::collections::HashMap;
use std::fs::read_to_string;

type Objects = HashMap<String, Vec<String>>;
#[derive(Debug)]
// Object contains its name and vector of other objects it orbits
struct Orbit(String, Vec<Orbit>);

fn traverse_orbits(objects: &mut Objects, object_name: &String) -> Orbit {
    let object_orbits: Vec<Orbit> = objects
        .get(object_name)
        .cloned()
        .map(|orbiters| {
            orbiters
                .iter()
                .map(|orbiter| traverse_orbits(objects, orbiter))
                .collect()
        })
        .unwrap_or(Vec::new());

    Orbit(object_name.clone(), object_orbits)
}

fn calculate_orbits_number(orbit: &Orbit, com_offset: usize) -> usize {
    orbit
        .1
        .iter()
        .map(|orbiter| calculate_orbits_number(orbiter, com_offset + 1))
        .sum::<usize>()
        + com_offset
}

fn main() -> std::io::Result<()> {
    let mut objects: Objects = HashMap::new();
    read_to_string("input.txt")?.lines().for_each(|line| {
        let orbited = line.get(0..3).expect("Invalid input");
        let orbiting = line.get(4..7).expect("Invalid input");

        match objects.get_mut(orbited) {
            Some(all) => {
                all.push(orbiting.to_owned());
            }
            None => {
                objects.insert(orbited.to_owned(), vec![orbiting.to_owned()]);
            }
        }

        match objects.get_mut(orbiting) {
            Some(all) => {
                all.push(orbited.to_owned());
            }
            None => {
                objects.insert(orbiting.to_owned(), vec![orbited.to_owned()]);
            }
        }
    });
    // create tree of orbits
    let orbits = traverse_orbits(&mut objects, &String::from("COM"));
    println!(
        "ANSWER TO FIRST PART: {:#?}",
        calculate_orbits_number(&orbits, 0)
    );

    let you_orbits = traverse_orbits(&mut objects, &String::from("YOU"));
    println!("{:#?}", you_orbits);
    Ok(())
}
