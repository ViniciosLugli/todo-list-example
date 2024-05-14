import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:dashboard/client/singleton.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:dashboard/src/rust/api/client.dart';

class TaskEditPage extends StatefulWidget {
  final Map<String, dynamic> task;

  const TaskEditPage({super.key, required this.task});

  @override
  _TaskEditPageState createState() => _TaskEditPageState();
}

class _TaskEditPageState extends State<TaskEditPage> {
  late TextEditingController _titleController;
  late TextEditingController _descriptionController;
  late bool _isTaskDone;
  late ApiClient _apiClient;

  @override
  void initState() {
    super.initState();
    _initializeApiClient();
    final task = widget.task;
    _titleController = TextEditingController(text: task['title']);
    _descriptionController = TextEditingController(text: task['description']);
    _isTaskDone = task['done'];
  }

  Future<void> _initializeApiClient() async {
    _apiClient = await Singleton.instance;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Edit Task'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              controller: _titleController,
              decoration: const InputDecoration(labelText: 'Title'),
            ),
            const SizedBox(height: 20),
            TextField(
              controller: _descriptionController,
              decoration: const InputDecoration(labelText: 'Description'),
            ),
            const SizedBox(height: 20),
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Text(
                  'Task Status:',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 10),
                ToggleButtons(
                  isSelected: [_isTaskDone, !_isTaskDone],
                  onPressed: (index) {
                    setState(() {
                      _isTaskDone = index == 0;
                    });
                  },
                  children: const [
                    Icon(Icons.check),
                    Icon(Icons.close),
                  ],
                ),
              ],
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: _saveTask,
              child: const Text('Save'),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _saveTask() async {
    final task = widget.task;

    try {
      var response = await _apiClient.updateTask(
        cuid: task["cuid"],
        title: _titleController.text,
        description: _descriptionController.text,
        done: _isTaskDone,
      );

      final json = jsonDecode(response);

      if (json['status_code'] != 200) {
        Fluttertoast.showToast(
          msg: json['error'],
          toastLength: Toast.LENGTH_SHORT,
          gravity: ToastGravity.BOTTOM,
          timeInSecForIosWeb: 1,
          backgroundColor: Colors.red,
          textColor: Colors.white,
          fontSize: 16.0,
        );
      } else {
        Navigator.pop(context, true);
      }
    } catch (e) {
      print('Error during task update: $e');
      Fluttertoast.showToast(
        msg: "Error during task update",
        toastLength: Toast.LENGTH_SHORT,
        gravity: ToastGravity.BOTTOM,
        timeInSecForIosWeb: 1,
        backgroundColor: Colors.red,
        textColor: Colors.white,
        fontSize: 16.0,
      );
    }
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }
}
