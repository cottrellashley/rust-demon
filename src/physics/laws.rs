use crate::physics::particles::{Particle};


/// A trait representing a pairwise interaction between two particles.
///
/// This trait defines a method to compute the effect of an interaction between two particles.
/// The interaction is applied only to the first particle (`obj1`), which is passed as a mutable reference,
/// while the second particle (`obj2`) is passed as an immutable reference.
///
/// # Details
///
/// - **Mutable vs. Immutable:**
///   `obj1` is mutable and its state will be updated as a result of the interaction.
///   `obj2` is immutable (read-only). In a simulation that iterates over all pairs,
///   you would eventually call the method with the roles reversed so that every particle is updated.
///
/// - **Return Value:**
///   The method returns a `Particle` representing the updated state of `obj1` after applying the interaction.
///   (This return value could be used to update the simulation state.)
///
/// # Examples
///
/// ```rust
/// // Assuming that CoulombLaw implements PairwiseInteraction:
/// let mut p1 = Particle::new(/* parameters here */);
/// let p2 = Particle::new(/* parameters here */);
/// let law = CoulombLaw::new(/* parameters here */);
/// p1 = law.resolve(&mut p1, &p2);
/// ```
///
pub trait InteractionLaw {
    /// Computes the interaction between two particles and returns the updated state for `obj1`.
    ///
    /// # Parameters
    ///
    /// - `obj1`: A mutable reference to the first particle. Its state will be updated based on the interaction.
    /// - `obj2`: An immutable reference to the second particle. Its state remains unchanged.
    ///
    /// # Returns
    ///
    /// Returns a `Particle` reflecting the updated state of `obj1` after the interaction.
    fn resolve(&self, obj1: &mut Particle, obj2: &mut Particle) -> bool;
}

/// A structure representing the Coulomb force interaction between charged particles.
///
/// Coulomb's law states that the magnitude of the electrostatic force between two point charges
/// is proportional to the product of the charges and inversely proportional to the square of the distance between them.
/// This struct encapsulates the constant and parameters needed to compute that force.
///
/// # Fields
///
/// * `k` - The Coulomb constant. Typically, this is \(8.9875517923 \times 10^9 \, \text{N·m}^2/\text{C}^2\).
/// * `softening` - A small value added (squared) to the denominator to avoid singularities when particles are extremely close.
/// * `cutoff` - A distance threshold beyond which the force is not applied.
///             (Particles farther apart than this value will not interact.)
///
/// # Example
///
/// ```rust
/// use crate::physics::coulomb_law::CoulombLaw;
/// use crate::physics::particles::Particle;
///
/// // Create a new CoulombLaw instance with a given constant, softening parameter, and cutoff.
/// let coulomb = CoulombLaw::new(8.9875517923e9, 0.001, 100.0);
///
/// // Later, you might use this instance to compute the interaction between two particles:
/// // coulomb.resolve(&mut particle1, &mut particle2);
/// ```
pub struct CoulombLaw {
    /// Coulomb's constant.
    pub k: f32,
    /// Softening parameter to prevent singularities when particles are extremely close.
    pub softening: f32,
    /// Distance cutoff; forces are only applied if the particle separation is below this value.
    pub cutoff: f32,
}

impl CoulombLaw {
    /// Creates a new instance of `CoulombLaw`.
    ///
    /// # Arguments
    ///
    /// * `k` - The Coulomb constant.
    /// * `softening` - A small value to be used as a softening parameter in the force calculation.
    /// * `cutoff` - The distance cutoff for applying the force.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::physics::coulomb_law::CoulombLaw;
    ///
    /// let coulomb = CoulombLaw::new(8.9875517923e9, 0.001, 100.0);
    /// ```
    pub fn new(k: f32, softening: f32, cutoff: f32) -> CoulombLaw {
        CoulombLaw { k, softening, cutoff }
    }
}

impl InteractionLaw for CoulombLaw {
    /// Resolves the Coulomb interaction between two particles.
    ///
    /// The force is only applied if the distance between `particle1` and `particle2`
    /// is below the cutoff value. The force is computed using Coulomb's law with a softening parameter,
    /// and the forces are applied so that like charges repel (and opposite charges attract).
    ///
    /// # Parameters
    ///
    /// * `particle1` - A mutable reference to the first particle.
    /// * `particle2` - A mutable reference to the second particle.
    ///
    /// # Returns
    ///
    /// Returns `true` if the interaction was computed (or skipped due to a zero distance), otherwise `false`.
    fn resolve(&self, particle1: &mut Particle, particle2: &mut Particle) -> bool {
        // Here we use hard-coded charges for demonstration.
        // In a full implementation, these would come from the Particle's properties.
        let p1_charge = 0.001234;
        let p2_charge = 0.001234;

        // Calculate the displacement vector from particle1 to particle2.
        let dx = particle2.position.x - particle1.position.x;
        let dy = particle2.position.y - particle1.position.y;
        let mut distance_sq = dx * dx + dy * dy;
        // Add softening to avoid singularity.
        distance_sq += self.softening * self.softening;
        let distance = distance_sq.sqrt();

        // Apply the cutoff: if particles are too far apart, do nothing.
        if distance > self.cutoff {
            return true;
        }

        if distance_sq < particle1.radius * 0.1 {
            distance_sq = particle1.radius * 0.1;
        }

        // Compute the force magnitude.
        // The sign of (p1_charge * p2_charge) will determine whether the force is attractive or repulsive.
        let force_magnitude = self.k * (p1_charge * p2_charge) / distance_sq;

        // For repulsion between like charges, the force on particle1 should be directed away from particle2.
        // Thus, subtract the force from particle1 and add it to particle2.
        particle1.force.x -= force_magnitude * dx / distance;
        particle1.force.y -= force_magnitude * dy / distance;

        particle2.force.x += force_magnitude * dx / distance;
        particle2.force.y += force_magnitude * dy / distance;

        true
    }
}

/// A structure representing impulse-based collision parameters for two-body interactions.
#[derive(Debug, Clone, Copy)]
pub struct ImpulseCollision {
    /// The coefficient of restitution (elasticity) of the collision.
    /// 1.0 is perfectly elastic, 0.0 is perfectly inelastic.
    pub restitution: f32,

    /// The positional correction factor (as a percentage of the penetration depth)
    /// used to resolve interpenetration after applying the impulse.
    pub correction_factor: f32,

    /// A small penetration threshold (slop) below which no positional correction is applied.
    /// This helps prevent jitter due to minor numerical inaccuracies.
    pub penetration_slop: f32,
}


impl ImpulseCollision {
    pub fn new(restitution: f32, correction_factor: f32, penetration_slop: f32) -> ImpulseCollision {
        ImpulseCollision {
            restitution: restitution,
            correction_factor: correction_factor,
            penetration_slop: penetration_slop
        }
    }
}

impl InteractionLaw for ImpulseCollision {

    fn resolve(&self, p1: &mut Particle, p2: &mut Particle) -> bool {
        let dx = p2.position.x - p1.position.x;
        let dy = p2.position.y - p1.position.y;
        let distance_sq = dx * dx + dy * dy;
        let radius_sum = p1.radius + p2.radius;

        if distance_sq < radius_sum * radius_sum {
            let distance = distance_sq.sqrt();
            // Avoid division by zero; if particles are on top of each other, skip collision resolution.
            if distance == 0.0 {
                return true;
            }
            // Normal vector (from self to other).
            let nx = dx / distance;
            let ny = dy / distance;

            // Relative velocity.
            let rvx = p1.velocity.x - p2.velocity.x;
            let rvy = p1.velocity.y - p2.velocity.y;
            // Dot product of relative velocity and normal.
            let rel_vel_dot_norm = rvx * nx + rvy * ny;

            // Only resolve if particles are moving toward each other.
            if rel_vel_dot_norm > 0.0 {
                return true;
            }

            // For equal mass and perfectly elastic collision:
            // The impulse scalar (simplified for m1 = m2 = 1).
            let impulse = rel_vel_dot_norm;
            // Update velocities.
            p1.velocity.x -= impulse * nx;
            p1.velocity.y -= impulse * ny;
            p2.velocity.x += impulse * nx;
            p2.velocity.y += impulse * ny;

            // Reposition particles so they are not overlapping.
            let overlap = 0.5 * (radius_sum - distance);
            p1.position.x -= overlap * nx;
            p1.position.y -= overlap * ny;
            p2.position.x += overlap * nx;
            p2.position.y += overlap * ny;
            return true;
        }
        return false;
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionLawType {
    CoulombLaw,
    ImpulseCollision
}


/// Creates and returns an interaction law instance based on the given `InteractionLawType`.
///
/// # Arguments
///
/// * `law_type` - An `InteractionLawType` enum indicating which interaction law to create.
///
/// # Returns
///
/// A boxed trait object (`Box<dyn InteractionLaw>`) that implements the desired interaction law.
///
/// # Examples
///
/// ```rust
/// // Create a Coulomb law interaction.
/// let law = build_interaction_law(InteractionLawType::CoulombLaw);
///
/// // Create an impulse collision interaction.
/// let impulse = build_interaction_law(InteractionLawType::ImpulseCollision);
/// ```
pub fn build_interaction_law(law_type: InteractionLawType) -> Box<dyn InteractionLaw> {
    match law_type {
        InteractionLawType::CoulombLaw => {
            // For example, use a Coulomb constant and softening parameter.
            Box::new(CoulombLaw::new(8.9875517923e9, 0.001, 2000.0))
        }
        InteractionLawType::ImpulseCollision => {
            // For example, use restitution, correction_factor, and penetration_slop.
            Box::new(ImpulseCollision::new(1.0, 0.8, 0.01))
        }
    }
}