import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:dashboard/client/singleton.dart';
import 'package:dashboard/pages/new_task.dart';
import 'package:dashboard/pages/edit_task.dart';
import 'package:dashboard/src/rust/api/client.dart';

class TaskList extends StatefulWidget {
  const TaskList({super.key});

  @override
  _TaskListState createState() => _TaskListState();
}

class _TaskListState extends State<TaskList> {
  late Future<List<Map<String, dynamic>>> _tasks;
  late ApiClient _apiClient;

  @override
  void initState() {
    super.initState();
    _refreshTasks();
  }

  Future<void> _refreshTasks() async {
    _initializeApiClient();

    setState(() {
      _tasks = _loadTasks();
    });
  }

  Future<void> _initializeApiClient() async {
    _apiClient = await Singleton.instance;
  }

  Future<List<Map<String, dynamic>>> _loadTasks() async {
    try {
      final response = await _apiClient.findAllTasks();

      final json = jsonDecode(response);
      if (json['status_code'] == 200) {
        final tasks = json['tasks'] as List<dynamic>;
        return tasks.cast<Map<String, dynamic>>();
      } else {
        Fluttertoast.showToast(
          msg: "Failed to load tasks",
          toastLength: Toast.LENGTH_SHORT,
          gravity: ToastGravity.BOTTOM,
          timeInSecForIosWeb: 1,
          backgroundColor: Colors.red,
          textColor: Colors.white,
          fontSize: 16.0,
        );
        return [];
      }
    } catch (e) {
      Fluttertoast.showToast(
        msg: "An error occurred: $e",
        toastLength: Toast.LENGTH_SHORT,
        gravity: ToastGravity.BOTTOM,
        timeInSecForIosWeb: 1,
        backgroundColor: Colors.red,
        textColor: Colors.white,
        fontSize: 16.0,
      );
      return [];
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Task List'),
      ),
      body: RefreshIndicator(
        onRefresh: _refreshTasks,
        child: FutureBuilder<List<Map<String, dynamic>>>(
          future: _tasks,
          builder: (context, snapshot) {
            if (snapshot.connectionState == ConnectionState.waiting) {
              return const Center(child: CircularProgressIndicator());
            } else if (snapshot.hasError) {
              return Center(child: Text('Error: ${snapshot.error}'));
            } else {
              final tasks = snapshot.data ?? [];
              return ListView.builder(
                itemCount: tasks.length,
                itemBuilder: (context, index) {
                  return TaskCard(
                    task: tasks[index],
                    refreshTasks: _refreshTasks,
                  );
                },
              );
            }
          },
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (context) => const NewTaskPage(),
            ),
          ).then((value) {
            if (value == true) {
              _refreshTasks();
            }
          });
        },
        child: const Icon(Icons.add),
      ),
    );
  }
}

class TaskCard extends StatelessWidget {
  final Map<String, dynamic> task;
  final VoidCallback refreshTasks;

  const TaskCard({super.key, required this.task, required this.refreshTasks});

  @override
  Widget build(BuildContext context) {
    final updatedAt =
        DateTime.parse(task['updatedAt']).toString().substring(0, 19);
    return Card(
      margin: const EdgeInsets.all(10),
      child: GestureDetector(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (context) => TaskEditPage(task: task),
            ),
          ).then((value) {
            if (value == true) {
              refreshTasks();
            }
          });
        },
        child: ListTile(
          title: Text(
            task['title'],
            style: const TextStyle(
              fontWeight: FontWeight.bold,
              fontSize: 16,
            ),
          ),
          subtitle: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                task['description'],
                style: const TextStyle(
                  fontSize: 14,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                'Status: ${task['done'] ? 'Done' : 'Not Done'}',
                style: TextStyle(
                  color: task['done'] ? Colors.green : Colors.red,
                  fontSize: 12,
                ),
              ),
              Text(
                'Last updated: $updatedAt',
                style: const TextStyle(
                  fontSize: 12,
                ),
              ),
            ],
          ),
          trailing: IconButton(
            icon: const Icon(Icons.delete),
            onPressed: () {
              showDialog(
                context: context,
                builder: (context) {
                  return AlertDialog(
                    title: const Text('Delete Task'),
                    content: const Text(
                        'Are you sure you want to delete this task?'),
                    actions: [
                      TextButton(
                        onPressed: () {
                          Navigator.pop(context);
                        },
                        child: const Text('Cancel'),
                      ),
                      TextButton(
                        onPressed: () async {
                          try {
                            final apiClient = await Singleton.instance;
                            final response =
                                await apiClient.deleteTask(cuid: task['cuid']);
                            final json = jsonDecode(response);
                            if (json['status_code'] == 200) {
                              Navigator.pop(context);
                              Fluttertoast.showToast(
                                msg: "Task deleted",
                                toastLength: Toast.LENGTH_SHORT,
                                gravity: ToastGravity.BOTTOM,
                                timeInSecForIosWeb: 1,
                                backgroundColor: Colors.green,
                                textColor: Colors.white,
                                fontSize: 16.0,
                              );
                              refreshTasks();
                            } else {
                              Fluttertoast.showToast(
                                msg: "Failed to delete task",
                                toastLength: Toast.LENGTH_SHORT,
                                gravity: ToastGravity.BOTTOM,
                                timeInSecForIosWeb: 1,
                                backgroundColor: Colors.red,
                                textColor: Colors.white,
                                fontSize: 16.0,
                              );
                            }
                          } catch (e) {
                            Fluttertoast.showToast(
                              msg: "An error occurred: $e",
                              toastLength: Toast.LENGTH_SHORT,
                              gravity: ToastGravity.BOTTOM,
                              timeInSecForIosWeb: 1,
                              backgroundColor: Colors.red,
                              textColor: Colors.white,
                              fontSize: 16.0,
                            );
                          }
                        },
                        child: const Text('Delete'),
                      ),
                    ],
                  );
                },
              );
            },
          ),
        ),
      ),
    );
  }
}
