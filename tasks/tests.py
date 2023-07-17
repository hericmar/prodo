from django.contrib.auth.models import User
from django.test import TestCase

from tasks.models import Task, TaskList


class TaskListTest(TestCase):
    @classmethod
    def setUpTestData(cls):
        cls.user = User.objects.create(username="TestUser", password="TestUser")

    def test_task_list_ordering(self):
        # task list should exist for every user
        self.assertEqual(TaskList.objects.filter(user=self.user).count(), 1)

        # Create 5 tasks
        task_summaries = ["Task 1", "Task 2", "Task 3", "Task 4", "Task 5"]
        tasks = []
        for summary in task_summaries:
            tasks.append(Task.objects.create(created_by=self.user, summary=summary))

        self.assertEqual(Task.objects.filter(created_by=self.user).count(), len(tasks))

        # Check that last created task is at the beginning of the list
        for i, task in enumerate(TaskList.objects.get(user=self.user).ordered_tasks):
            self.assertEqual(task, str(tasks[len(tasks) - 1 - i].uid))

        # Move task 3 to the top
        tasks[2].save_order(0)
        self.assertEqual(TaskList.objects.get(user=self.user).ordered_tasks[0], str(tasks[2].uid))

        # Remove task 3
        tasks[2].delete()
        task_list = TaskList.objects.get(user=self.user)
        self.assertEqual(len(task_list.ordered_tasks), len(tasks) - 1)
        self.assertEqual(task_list.ordered_tasks[0], str(tasks[4].uid))
        self.assertEqual(task_list.ordered_tasks[1], str(tasks[3].uid))
        self.assertEqual(task_list.ordered_tasks[2], str(tasks[1].uid))
        self.assertEqual(task_list.ordered_tasks[3], str(tasks[0].uid))
