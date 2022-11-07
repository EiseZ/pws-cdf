struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

struct Particle {
    position: Vector<f32>,
    velocity: f32,
    density: f32,
}

const TIMESTEP: f32 = 0.01; // TODO: Find right value for this
const TIME_END: f32 = 10.;
const NEIGBOUR_RANGE: f32 = 5.;
const SMOOTHING_LENGTH: f32 = 1.; // TODO: Find right value for this
const PARTICLE_MASS: f32 = 1.; // TODO: Find right value for this
const PRESSURE_CONSTANT_K: f32 = 0.01; // TODO: Find right value for this
const PRESSURE_CONSTANT_γ: u8 = 1; // TODO: Find right value for this
const REFERENCE_DENSITY: f32 = 1.; // TODO: Find right value for this

fn main() {
    let mut current_time = 0.;
    let mut particles = vec![Particle { position: Vector { x: 1., y: 0., z: 0. }, velocity: 1., density: 1. }];

    while current_time < TIME_END {
        let neigbourhoods = find_neigbourhoods(NEIGBOUR_RANGE, &particles);
        let densities = calculate_densities(&neigbourhoods, &particles);
        let pressures = calculate_pressures(&densities, &particles, PRESSURE_CONSTANT_K, PRESSURE_CONSTANT_γ, REFERENCE_DENSITY);

        current_time += TIMESTEP;
    }
}

fn find_neigbourhoods(range: f32, particles: &Vec<Particle>) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = Vec::new();
    for particle in particles {
        let mut neigbours: Vec<usize> = Vec::new();
        for possible_neigbour_n in 1..particles.len() {
            if distance(&particle.position, &particles[possible_neigbour_n].position) < range {
                neigbours.push(possible_neigbour_n);
            }
        }
        res.push(neigbours);
    }
    res
}

fn calculate_densities(neigbourhoods: &Vec<Vec<usize>>, particles: &Vec<Particle>) -> Vec<f32> {
    let res: Vec<f32> = Vec::new();
    for (i, neigbourhood) in neigbourhoods.iter().enumerate() {
        let mut density: f32 = 0.;
        for neigbour_n in neigbourhood {
            kernel(particles[i].position.x - particles[neigbour_n.clone()].position.x, SMOOTHING_LENGTH);
        }
        density *= PARTICLE_MASS;
        res.push(density);
    }
    res
}

fn calculate_pressures(densities: &Vec<f32>, particles: &Vec<Particle>, pressure_constant_k: f32, pressure_constant_γ: u8, reference_density: f32) -> Vec<f32> {
    let res: Vec<f32> = Vec::new();
    for density in densities {
        let mut pressure: f32 = ((pressure_constant_k * reference_density) / pressure_constant_γ) * ((density / reference_density).powi(pressure_constant_γ) - 1);
        res.push(pressure);
    }
    res
}

fn distance(p1: &Vector<f32>, p2: &Vector<f32>) -> f32 {
    f32::sqrt((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2))
}

fn kernel(distance: f32, smoothing_lenght: f32) -> f32 {
    // Source: http://www.plunk.org/~trina/thesis/html/thesis_ch2.html#equ2.5
    let q: f32 = distance / smoothing_lenght;
    if 0. <= distance && distance <= 1. {
        1. + ((3./2.) * q.powi(2)) + ((3./4.) * q.powi(3))
    } else if 1. <= q && q <= 2. {
        (1./4.) * (2. - q).powi(3)
    } else {
        0.
    }
}

// Sources:
// - https://people.inf.ethz.ch/~sobarbar/papers/Sol09/Sol09.pdf
// - http://www.plunk.org/~trina/thesis/html/thesis_ch2.html
