import datetime

from django.contrib.auth.models import User
from django.test import TestCase
from django.utils import timezone

from tasks.models import Task, TaskList, TASK_URGENCY_NONE, TASK_PRIORITY_LOW, TASK_PRIORITY_MEDIUM, TASK_PRIORITY_HIGH, \
    TASK_URGENCY_LOW, TASK_URGENCY_MEDIUM, TASK_URGENCY_HIGH
from tasks.services import calculate_score
from users.models import UserData


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


class TaskUrgencyTest(TestCase):
    @classmethod
    def setUpTestData(cls):
        cls.user = User.objects.create(username="TestUser", password="TestUser")

    def test_calculate_urgency(self):
        now = timezone.now()
        calculate_score(now)

        test_user_data = UserData.objects.get(user=self.user)
        self.assertEqual(test_user_data.urgency_calculated_at, now)

    def test_on_task_with_no_recurrence_and_no_due_date(self):
        test_data = (
            ('2023-07-15T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-15T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_NONE),
            ('2023-07-15T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_NONE),

            ('2023-07-16T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-16T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_NONE),
            ('2023-07-16T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_LOW),

            ('2023-07-17T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-17T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_LOW),
            ('2023-07-17T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_MEDIUM),

            ('2023-07-18T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-18T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_LOW),
            ('2023-07-18T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-19T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-19T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_MEDIUM),
            ('2023-07-19T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-20T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-20T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_MEDIUM),
            ('2023-07-20T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-21T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-21T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-21T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-22T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-22T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-22T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-23T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-23T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-23T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            ('2023-07-24T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-24T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-24T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),
        )

        now = datetime.datetime.strptime(test_data[0][0], '%Y-%m-%dT%H:%M')
        now = timezone.make_aware(now, timezone.get_current_timezone())
        task = Task.objects.create(created_by=self.user, created=now, summary="Test task")
        task.created = now

        for test in test_data:
            now = datetime.datetime.strptime(test[0], '%Y-%m-%dT%H:%M')
            now = timezone.make_aware(now, timezone.get_current_timezone())
            task.priority = test[1]
            task.calculate_urgency(now)
            self.assertEqual(task.urgency, test[2], 'Failed on %s with priority %s' % (test[0], test[1]))

    def test_on_task_with_due_date(self):
        # created, due, priority, urgency
        test_data = (
            # due yesterday
            ('2023-07-15T10:00', '2023-07-14T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-14T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-14T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            # due now
            ('2023-07-15T10:00', '2023-07-15T10:00', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-15T10:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-15T10:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            # due today
            ('2023-07-15T10:00', '2023-07-15T23:00', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-15T23:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-15T10:00', '2023-07-15T23:00', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),

            # due tomorrow (more than 24 hours)
            ('2023-07-16T11:00', '2023-07-17T12:00', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-16T11:00', '2023-07-17T12:00', TASK_PRIORITY_MEDIUM, TASK_URGENCY_MEDIUM),
            ('2023-07-16T11:00', '2023-07-17T12:00', TASK_PRIORITY_HIGH, TASK_URGENCY_MEDIUM),
        )
        now = datetime.datetime.strptime('2023-07-15T10:00', '%Y-%m-%dT%H:%M')
        now = timezone.make_aware(now, timezone.get_current_timezone())
        task = Task.objects.create(created_by=self.user, created=now, summary="Test task")
        task.created = now

        for test in test_data:
            now = datetime.datetime.strptime(test[0], '%Y-%m-%dT%H:%M')
            now = timezone.make_aware(now, timezone.get_current_timezone())
            due = datetime.datetime.strptime(test[1], '%Y-%m-%dT%H:%M')
            due = timezone.make_aware(due, timezone.get_current_timezone())
            task.due = due
            task.priority = test[2]
            task.calculate_urgency(now)
            self.assertEqual(task.urgency, test[3], 'Failed on %s with priority %s' % (test[0], test[2]))

    def test_on_task_with_rrule(self):
        # created, rrule, priority, urgency
        test_data = (
            ('2023-07-15T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-16T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-17T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_NONE),
            ('2023-07-18T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-19T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-20T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_LOW),
            ('2023-07-21T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-22T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-23T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_MEDIUM),
            ('2023-07-24T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-25T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),
            ('2023-07-26T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_LOW, TASK_URGENCY_HIGH),

            ('2023-07-15T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_NONE),
            ('2023-07-16T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_NONE),
            ('2023-07-17T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_LOW),
            ('2023-07-18T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_LOW),
            ('2023-07-19T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_MEDIUM),
            ('2023-07-20T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_MEDIUM),
            ('2023-07-21T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),
            ('2023-07-22T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_MEDIUM, TASK_URGENCY_HIGH),

            ('2023-07-15T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_HIGH, TASK_URGENCY_NONE),
            ('2023-07-16T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_HIGH, TASK_URGENCY_LOW),
            ('2023-07-17T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_HIGH, TASK_URGENCY_MEDIUM),
            ('2023-07-18T10:00', 'FREQ=DAILY;INTERVAL=1', TASK_PRIORITY_HIGH, TASK_URGENCY_HIGH),
        )
        now = datetime.datetime.strptime('2023-07-15T10:00', '%Y-%m-%dT%H:%M')
        now = timezone.make_aware(now, timezone.get_current_timezone())
        task = Task.objects.create(created_by=self.user, created=now, summary="Test task")
        task.created = now

        for test in test_data:
            now = datetime.datetime.strptime(test[0], '%Y-%m-%dT%H:%M')
            now = timezone.make_aware(now, timezone.get_current_timezone())
            task.rrule = test[1]
            task.priority = test[2]
            task.calculate_urgency(now)
            self.assertEqual(task.urgency, test[3], 'Failed on %s with priority %s' % (test[0], test[2]))
