import 'package:flutter/material.dart';
import 'core/services/simulation_service.dart';

void main() {
  runApp(const AquaRocketApp());
}

class AquaRocketApp extends StatelessWidget {
  const AquaRocketApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'AquaRocket Mission Control',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        brightness: Brightness.dark,
        colorSchemeSeed: Colors.cyan,
        useMaterial3: true,
      ),
      home: const DashboardScreen(),
    );
  }
}

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  final AquaRocketService _service = AquaRocketService();
  SimulationResult? _result;

  double _radius = 0.025;
  double _s1Impulse = 20.0;
  double _s2Impulse = 18.0;

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

  @override
  void initState() {
    super.initState();
    _runSimulation();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('AquaRocket 6-DOF Mission Control'),
        centerTitle: true,
        backgroundColor: Colors.black45,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (_result != null) ...[
              Card(
                elevation: 4,
                color: const Color(0xFF161B22),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    children: [
                      const Text(
                        "FLIGHT TELEMETRY (RUST CORE)",
                        style: TextStyle(fontWeight: FontWeight.bold, color: Colors.cyanAccent),
                      ),
                      const Divider(color: Colors.white24),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceAround,
                        children: [
                          _statTile("Apogee", "${_result!.apogee.toStringAsFixed(1)} m"),
                          _statTile("Max Speed", "${_result!.maxVelocity.toStringAsFixed(1)} m/s"),
                          _statTile("Landing", "${_result!.landingVelocity.toStringAsFixed(2)} m/s"),
                        ],
                      ),
                      const SizedBox(height: 12),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceAround,
                        children: [
                          _statTile("S1 Separation", "${_result!.stage1BurnoutAlt.toStringAsFixed(1)} m"),
                          _statTile("S2 Burnout", "${_result!.stage2BurnoutAlt.toStringAsFixed(1)} m"),
                          _statTile("Total Flight", "${_result!.totalFlightTime.toStringAsFixed(1)} s"),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 20),
            ],
            Card(
              elevation: 2,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text("Body Radius: ${(_radius * 1000).toStringAsFixed(1)} mm"),
                    Slider(
                      value: _radius,
                      min: 0.015,
                      max: 0.050,
                      divisions: 35,
                      onChanged: (v) {
                        setState(() => _radius = v);
                        _runSimulation();
                      },
                    ),
                    Text("Stage 1 Booster Impulse: ${_s1Impulse.toStringAsFixed(1)} N·s"),
                    Slider(
                      value: _s1Impulse,
                      min: 10.0,
                      max: 60.0,
                      divisions: 50,
                      onChanged: (v) {
                        setState(() => _s1Impulse = v);
                        _runSimulation();
                      },
                    ),
                    Text("Stage 2 Sustainer Impulse: ${_s2Impulse.toStringAsFixed(1)} N·s"),
                    Slider(
                      value: _s2Impulse,
                      min: 10.0,
                      max: 50.0,
                      divisions: 40,
                      onChanged: (v) {
                        setState(() => _s2Impulse = v);
                        _runSimulation();
                      },
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _statTile(String label, String value) {
    return Column(
      children: [
        Text(label, style: const TextStyle(fontSize: 12, color: Colors.grey)),
        const SizedBox(height: 4),
        Text(value, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
      ],
    );
  }
}
