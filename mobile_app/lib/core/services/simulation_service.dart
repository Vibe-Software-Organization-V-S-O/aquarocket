import 'dart:ffi';
import 'package:ffi/ffi.dart';
import '../ffi/aquarocket_bindings.dart';

class SimulationResult {
  final double apogee;
  final double maxVelocity;
  final double stage1BurnoutAlt;
  final double stage1SeparationTime;
  final double stage2BurnoutAlt;
  final double timeToApogee;
  final double totalFlightTime;
  final double landingVelocity;

  SimulationResult({
    required this.apogee,
    required this.maxVelocity,
    required this.stage1BurnoutAlt,
    required this.stage1SeparationTime,
    required this.stage2BurnoutAlt,
    required this.timeToApogee,
    required this.totalFlightTime,
    required this.landingVelocity,
  });
}

class AquaRocketService {
  final AquaRocketNativeBridge _bridge = AquaRocketNativeBridge();

  SimulationResult simulate({
    required double radius,
    required double dragCoeff,
    required double s1DryMass,
    required double s1BurnTime,
    required double s1Impulse,
    required double s1PropellantMass,
    required double s2DryMass,
    required double s2BurnTime,
    required double s2Impulse,
    required double s2PropellantMass,
    required double s2IgnitionDelay,
    required double parachuteDiameter,
  }) {
    final arena = Arena();
    try {
      final config = arena<CRocketConfig>();
      config.ref.radius = radius;
      config.ref.dragCoeff = dragCoeff;
      config.ref.s1DryMass = s1DryMass;
      config.ref.s1BurnTime = s1BurnTime;
      config.ref.s1Impulse = s1Impulse;
      config.ref.s1PropellantMass = s1PropellantMass;
      config.ref.s2DryMass = s2DryMass;
      config.ref.s2BurnTime = s2BurnTime;
      config.ref.s2Impulse = s2Impulse;
      config.ref.s2PropellantMass = s2PropellantMass;
      config.ref.s2IgnitionDelay = s2IgnitionDelay;
      config.ref.parachuteDiameter = parachuteDiameter;

      final res = _bridge.runSimulation(config.ref);

      return SimulationResult(
        apogee: res.apogee,
        maxVelocity: res.maxVelocity,
        stage1BurnoutAlt: res.stage1BurnoutAlt,
        stage1SeparationTime: res.stage1SeparationTime,
        stage2BurnoutAlt: res.stage2BurnoutAlt,
        timeToApogee: res.timeToApogee,
        totalFlightTime: res.totalFlightTime,
        landingVelocity: res.landingVelocity,
      );
    } finally {
      arena.releaseAll();
    }
  }
}
