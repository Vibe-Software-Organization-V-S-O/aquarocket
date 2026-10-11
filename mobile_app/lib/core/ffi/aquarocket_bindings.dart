import 'dart:ffi';
import 'dart:io';

final class CRocketConfig extends Struct {
  @Float()
  external double radius;

  @Float()
  external double dragCoeff;

  @Float()
  external double s1DryMass;

  @Float()
  external double s1BurnTime;

  @Float()
  external double s1Impulse;

  @Float()
  external double s1PropellantMass;

  @Float()
  external double s2DryMass;

  @Float()
  external double s2BurnTime;

  @Float()
  external double s2Impulse;

  @Float()
  external double s2PropellantMass;

  @Float()
  external double s2IgnitionDelay;

  @Float()
  external double parachuteDiameter;
}

final class CRocketTelemetry extends Struct {
  @Float()
  external double apogee;

  @Float()
  external double maxVelocity;

  @Float()
  external double stage1BurnoutAlt;

  @Float()
  external double stage1SeparationTime;

  @Float()
  external double stage2BurnoutAlt;

  @Float()
  external double timeToApogee;

  @Float()
  external double totalFlightTime;

  @Float()
  external double landingVelocity;
}

typedef SimulateFlightC = CRocketTelemetry Function(CRocketConfig config);
typedef SimulateFlightDart = CRocketTelemetry Function(CRocketConfig config);

class AquaRocketNativeBridge {
  late final DynamicLibrary _lib;
  late final SimulateFlightDart _simulateFlight;

  AquaRocketNativeBridge() {
    _lib = Platform.isAndroid
        ? DynamicLibrary.open("libaquarocket.so")
        : Platform.isLinux
            ? DynamicLibrary.open("./target/release/libaquarocket.so")
            : DynamicLibrary.process();

    _simulateFlight = _lib
        .lookup<NativeFunction<SimulateFlightC>>('aquarocket_simulate_flight')
        .asFunction<SimulateFlightDart>();
  }

  CRocketTelemetry runSimulation(CRocketConfig config) {
    return _simulateFlight(config);
  }
}
