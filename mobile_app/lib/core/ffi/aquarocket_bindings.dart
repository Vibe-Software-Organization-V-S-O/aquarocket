import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

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

typedef GenerateMeshC = Bool Function(
  Pointer<Utf8> outputPath,
  Float caliber,
  Float noseLen,
  Float bodyLen,
  Float finRoot,
  Float finTip,
  Float finSpan,
  Float finSweep,
  Uint32 finCount,
);

typedef GenerateMeshDart = bool Function(
  Pointer<Utf8> outputPath,
  double caliber,
  double noseLen,
  double bodyLen,
  double finRoot,
  double finTip,
  double finSpan,
  double finSweep,
  int finCount,
);

class AquaRocketNativeBridge {
  late final DynamicLibrary _lib;
  late final SimulateFlightDart _simulateFlight;
  late final GenerateMeshDart _generateMesh;

  AquaRocketNativeBridge() {
    _lib = Platform.isAndroid
        ? DynamicLibrary.open("libaquarocket.so")
        : Platform.isLinux
            ? DynamicLibrary.open("./target/release/libaquarocket.so")
            : DynamicLibrary.process();

    _simulateFlight = _lib
        .lookup<NativeFunction<SimulateFlightC>>('aquarocket_simulate_flight')
        .asFunction<SimulateFlightDart>();

    _generateMesh = _lib
        .lookup<NativeFunction<GenerateMeshC>>('aquarocket_generate_cad_mesh')
        .asFunction<GenerateMeshDart>();
  }

  CRocketTelemetry runSimulation(CRocketConfig config) {
    return _simulateFlight(config);
  }

  bool generateCadMesh(
    String path,
    double caliber,
    double noseLen,
    double bodyLen,
    double finRoot,
    double finTip,
    double finSpan,
    double finSweep,
    int finCount,
  ) {
    final pathPtr = path.toNativeUtf8();
    final res = _generateMesh(
      pathPtr,
      caliber,
      noseLen,
      bodyLen,
      finRoot,
      finTip,
      finSpan,
      finSweep,
      finCount,
    );
    calloc.free(pathPtr);
    return res;
  }
}
