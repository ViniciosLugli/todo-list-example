import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:dashboard/src/rust/api/client.dart';

class ApiClientInstance {
  static ApiClient? _instance;
  final String baseUrl;

  ApiClientInstance._({required this.baseUrl});

  static Future<ApiClient> newInstance({required String baseUrl}) async {
    _instance ??= await ApiClient.newInstance(baseUrl: baseUrl);
    return _instance!;
  }
}

class Singleton {
  static Future<ApiClient> get instance async {
    final apiUrl = dotenv.env['API_URL'];

    if (apiUrl == null) {
      throw Exception('API_URL is not set');
    }

    return await ApiClientInstance.newInstance(baseUrl: apiUrl);
  }
}
