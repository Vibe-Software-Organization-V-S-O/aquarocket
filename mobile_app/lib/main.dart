import 'package:flutter/material.dart';
import 'package:flutter_cube/flutter_cube.dart';
import 'core/services/simulation_service.dart';

void main() {
  runApp(const AquaRocketApp());
}

class AquaRocketApp extends StatelessWidget {
  const AquaRocketApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'AquaRocket CAD & Sim',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF070B11),
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF00F0FF),
          secondary: Color(0xFF00B4D8),
          surface: Color(0xFF0D1520),
        ),
      ),
      home: const RocketStudioDashboard(),
    );
  }
}

class RocketStudioDashboard extends StatefulWidget {
  const RocketStudioDashboard({super.key});

  @override
  State<RocketStudioDashboard> createState() => _RocketStudioDashboardState();
}

class _RocketStudioDashboardState extends State<RocketStudioDashboard> {
  final AquaRocketService _service = AquaRocketService();
  SimulationResult? _result;
  late Scene _scene;
  Object? _rocketObject;

  // প্যারামেট্রিক ভ্যারিয়েবলস
  double _radius = 0.025;
  double _s1Impulse = 25.0;
  double _s2Impulse = 15.0;

  @override
  void initState() {
    super.initState();
    _runSimulation();
  }

  void _runSimulation() {
    final res = _service.simulate(
      radius: _radius,
      dragCoeff: 0.42,
      s1DryMass: 0.080,
      s1BurnTime: 1.2,
      s1Impulse: _s1Impulse,
      s1PropellantMass: 0.035,
      s2DryMass: 0.090,
      s2BurnTime: 2.0,
      s2Impulse: _s2Impulse,
      s2PropellantMass: 0.028,
      s2IgnitionDelay: 0.5,
      parachuteDiameter: 0.35,
    );

    setState(() {
      _result = res;
    });
  }

  void _onSceneCreated(Scene scene) {
    _scene = scene;
    scene.camera.position.setValues(0, 0, 4.5);
    scene.camera.target.setValues(0, 0, 0);

    _rocketObject = Object(
      fileName: 'assets/models/rocket.obj',
      isAsset: true,
      scale: Vector3(1.2, 1.2, 1.2),
      lighting: true,
    );
    scene.world.add(_rocketObject!);
  }

  @override
  Widget build(BuildContext context) {
    const aquaColor = Color(0xFF00F0FF);

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          "AQUAROCKET // 3D CAD STUDIO",
          style: TextStyle(
            color: aquaColor,
            fontWeight: FontWeight.bold,
            letterSpacing: 2.0,
            fontSize: 16,
          ),
        ),
        backgroundColor: const Color(0xFF0D1520),
        elevation: 0,
        centerTitle: true,
      ),
      body: Column(
        children: [
          // ১. ৩৬০° লাইভ ৩D ভিউপোর্ট
          Expanded(
            flex: 4,
            child: Container(
              margin: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: const Color(0xFF0A1017),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: aquaColor.withOpacity(0.4), width: 1.5),
                boxShadow: [
                  BoxShadow(
                    color: aquaColor.withOpacity(0.15),
                    blurRadius: 15,
                    spreadRadius: 2,
                  ),
                ],
              ),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(16),
                child: Stack(
                  children: [
                    Cube(onSceneCreated: _onSceneCreated),
                    Positioned(
                      top: 12,
                      left: 12,
                      child: Container(
                        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                        decoration: BoxDecoration(
                          color: Colors.black.withOpacity(0.6),
                          borderRadius: BorderRadius.circular(20),
                          border: Border.all(color: aquaColor.withOpacity(0.5)),
                        ),
                        child: const Row(
                          children: [
                            Icon(Icons.threed_rotation, color: aquaColor, size: 16),
                            SizedBox(width: 6),
                            Text(
                              "INTERACTIVE 3D CAD",
                              style: TextStyle(color: aquaColor, fontSize: 11, fontWeight: FontWeight.w600),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),

          // ২. টেলিমেট্রি কার্ড ও অ্যারোস্পেস কন্ট্রোলস
          Expanded(
            flex: 5,
            child: SingleChildScrollView(
              padding: const EdgeInsets.symmetric(horizontal: 14),
              child: Column(
                children: [
                  if (_result != null) ...[
                    Container(
                      padding: const EdgeInsets.all(14),
                      decoration: BoxDecoration(
                        color: const Color(0xFF0E1724),
                        borderRadius: BorderRadius.circular(14),
                        border: Border.all(color: const Color(0xFF1E2D42)),
                      ),
                      child: Column(
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              const Text("TELEMETRY", style: TextStyle(color: aquaColor, fontWeight: FontWeight.bold, fontSize: 12)),
                              Text("APOGEE: ${_result!.apogee.toStringAsFixed(1)} M", style: const TextStyle(color: aquaColor, fontWeight: FontWeight.bold, fontSize: 13)),
                            ],
                          ),
                          const Divider(color: Colors.white10, height: 16),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceAround,
                            children: [
                              _metricBox("Max Speed", "${_result!.maxVelocity.toStringAsFixed(1)} m/s"),
                              _metricBox("S1 Sep", "${_result!.stage1BurnoutAlt.toStringAsFixed(1)} m"),
                              _metricBox("Flight Time", "${_result!.totalFlightTime.toStringAsFixed(1)} s"),
                            ],
                          ),
                        ],
                      ),
                    ),
                    const SizedBox(height: 12),
                  ],

                  // প্যারামেট্রিক ডিজাইনার স্লাইডার
                  Container(
                    padding: const EdgeInsets.all(14),
                    decoration: BoxDecoration(
                      color: const Color(0xFF0E1724),
                      borderRadius: BorderRadius.circular(14),
                      border: Border.all(color: const Color(0xFF1E2D42)),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("PARAMETRIC AIRFRAME CONFIG", style: TextStyle(color: Colors.white70, fontSize: 11, fontWeight: FontWeight.bold)),
                        const SizedBox(height: 10),
                        _sliderTile("Caliber (Diameter)", "${(_radius * 2000).toStringAsFixed(1)} mm", _radius, 0.015, 0.050, (v) {
                          setState(() => _radius = v);
                          _runSimulation();
                        }),
                        _sliderTile("Booster Impulse", "${_s1Impulse.toStringAsFixed(1)} N·s", _s1Impulse, 10.0, 60.0, (v) {
                          setState(() => _s1Impulse = v);
                          _runSimulation();
                        }),
                        _sliderTile("Sustainer Impulse", "${_s2Impulse.toStringAsFixed(1)} N·s", _s2Impulse, 10.0, 50.0, (v) {
                          setState(() => _s2Impulse = v);
                          _runSimulation();
                        }),
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _metricBox(String title, String val) {
    return Column(
      children: [
        Text(title, style: const TextStyle(color: Colors.white54, fontSize: 11)),
        const SizedBox(height: 4),
        Text(val, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 14)),
      ],
    );
  }

  Widget _sliderTile(String label, String displayVal, double value, double min, double max, ValueChanged<double> onChanged) {
    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(label, style: const TextStyle(color: Colors.white60, fontSize: 12)),
            Text(displayVal, style: const TextStyle(color: Color(0xFF00F0FF), fontSize: 12, fontWeight: FontWeight.bold)),
          ],
        ),
        SliderTheme(
          data: SliderTheme.of(context).copyWith(
            activeTrackColor: const Color(0xFF00F0FF),
            thumbColor: const Color(0xFF00F0FF),
            inactiveTrackColor: Colors.white12,
            trackHeight: 2,
          ),
          child: Slider(value: value, min: min, max: max, onChanged: onChanged),
        ),
      ],
    );
  }
}
