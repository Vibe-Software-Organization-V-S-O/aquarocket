#include <stdio.h>
#include "aquarocket.h"

int main() {
    printf("--- Testing AquaRocket C-ABI Shared Object ---\n");

    struct CRocketConfig cfg = {
        .radius = 0.025f,
        .drag_coeff = 0.42f,
        .s1_dry_mass = 0.080f,
        .s1_burn_time = 1.2f,
        .s1_impulse = 20.0f,
        .s1_propellant_mass = 0.035f,
        .s2_dry_mass = 0.090f,
        .s2_burn_time = 2.0f,
        .s2_impulse = 18.0f,
        .s2_propellant_mass = 0.028f,
        .s2_ignition_delay = 0.5f,
        .parachute_diameter = 0.35f
    };

    struct CRocketTelemetry result = aquarocket_simulate_flight(cfg);

    printf("Telemetry from Rust lib:\n");
    printf("  Apogee Altitude : %.2f m\n", result.apogee);
    printf("  Max Velocity    : %.2f m/s\n", result.max_velocity);
    printf("  Landing Speed   : %.2f m/s\n", result.landing_velocity);

    return 0;
}
