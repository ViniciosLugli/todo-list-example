import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:dashboard/client/singleton.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:flutter/material.dart';

class TaskEditPage extends StatefulWidget {
  final Map<String, dynamic> task;

  const TaskEditPage({super.key, required this.task});

  @override
  _TaskEditPageState createState() => _TaskEditPageState();
}

class _TaskEditPageState extends State<TaskEditPage> {
  late TextEditingController _titleController;
  late TextEditingController _descriptionController;

  @override
  void initState() {
    super.initState();
    final task = widget.task;
    _titleController = TextEditingController(text: task['title']);
    _descriptionController = TextEditingController(text: task['description']);
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
                  isSelected: [widget.task['done'], !widget.task['done']],
                  onPressed: (index) {
                    setState(() {
                      widget.task['done'] = index == 0;
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
              onPressed: () async {
                final task = widget.task;

                final apiClient = await Singleton.instance;
                var response = await apiClient.updateTask(
                    cuid: task["cuid"],
                    title: _titleController.text,
                    description: _descriptionController.text,
                    done: task["done"]);

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
                  return;
                } else {
                  Navigator.pop(context, true);
                }
              },
              child: const Text('Save'),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }
}
