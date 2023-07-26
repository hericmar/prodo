import math
import uuid

from dateutil import rrule
from django.contrib.auth.models import User
from django.db import models
from django.core.exceptions import ValidationError
from django.db.models.signals import post_save
from django.dispatch import receiver
from django.utils import timezone

from base.models import Creatable, Updatable


def validate_rrule(value: str):
    """
    Validate the rrule field
    """
    try:
        rrule.rrulestr(value)
    except ValueError as err:
        raise ValidationError(str(err), params={'value': value})


def make_daylong(self, date):
    self.start = date
    self.end = None


class TaskManager(models.Manager):
    def get_ordered(self, user: User):
        task_list = TaskList.objects.get(user=user)
        uuids = [uuid.UUID(uid) for uid in task_list.ordered_tasks]
        filtered_tasks = self.filter(created_by=user, active=True)

        return sorted(filtered_tasks, key=lambda task: uuids.index(task.uid))

    def get_active(self, user: User):
        return self.filter(created_by=user, active=True)

    def create(self, *args, **kwargs):
        task = super(TaskManager, self).create(*args, **kwargs)
        task.calculate_urgency(timezone.now())

        task_list = TaskList.objects.filter(user=task.created_by).first()
        task_list.ordered_tasks.insert(0, str(task.uid))
        task_list.save()

        return task


# iCalendar RFC 5545 values
TASK_PRIORITY_HIGH = 1
TASK_PRIORITY_MEDIUM = 5
TASK_PRIORITY_LOW = 9

TASK_PRIORITY_CHOICES = (
    (TASK_PRIORITY_HIGH, "High"),
    (TASK_PRIORITY_MEDIUM, "Medium"),
    (TASK_PRIORITY_LOW, "Low"),
)

TASK_PRIORITY_TO_INDEX = {
    TASK_PRIORITY_HIGH: 1,
    TASK_PRIORITY_MEDIUM: 2,
    TASK_PRIORITY_LOW: 3,
}

# no color - default for a task after creation
TASK_URGENCY_NONE = 0
# yellow - task is not urgent
TASK_URGENCY_LOW = 1
# orange - task should be completed soon
TASK_URGENCY_MEDIUM = 2
# red - missed due, task should be completed as soon as possible
TASK_URGENCY_HIGH = 3


class Task(Creatable, Updatable):
    """
    Partially derived from RFC 5545
    """
    uid = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    summary = models.CharField(max_length=60)
    description = models.TextField(default="", blank=True)
    start = models.DateTimeField(null=True, blank=True)
    end = models.DateTimeField(null=True, blank=True)
    due = models.DateTimeField(null=True, blank=True)
    rrule = models.CharField(max_length=255, validators=[validate_rrule], null=True, blank=True)
    sequence = models.PositiveIntegerField(default=0)
    active = models.BooleanField(default=True)
    priority = models.PositiveSmallIntegerField(default=TASK_PRIORITY_LOW, choices=TASK_PRIORITY_CHOICES)

    urgency = models.PositiveSmallIntegerField(default=TASK_URGENCY_NONE)

    # Calculated score for the task
    # Based on the priority and the time left to complete the task
    score = models.FloatField(default=0.0)

    # Last time the task was completed
    completed = models.DateTimeField(null=True, blank=True)

    # This is a custom manager that will be used instead of the default one
    objects = TaskManager()

    def calculate_urgency(self, now) -> None:
        """
        Not saved to the database
        """
        if self.due:
            pass
        elif self.rrule:
            recurrence = rrule.rrulestr(self.rrule)
            next = recurrence.after(now)
        else:
            # hours passed since the task was created
            hours = (now - self.created).total_seconds() / 3600
            step_size = 24 * TASK_PRIORITY_TO_INDEX[self.priority]
            self.urgency = min(math.floor(float(hours) / float(step_size)), TASK_URGENCY_HIGH)

    def save(self, *args, **kwargs):
        """
        update sequence number when task is updated in the database
        """
        try:
            previous = Task.objects.get(uid=self.uid)

            if not previous.active:
                # TODO check if task is not already inactive, must be done in the database
                raise ValidationError('Task is not active')

            if previous.created != self.created:
                raise ValidationError('Task created date cannot be modified')

            self.sequence += 1
        except Task.DoesNotExist:
            pass

        super(Task, self).save(*args, **kwargs)

        self.calculate_urgency(timezone.now())

    def save_inactive(self):
        self.active = False
        self.save()

    def save_order(self, order: int):
        if not self.active:
            raise ValidationError('Task is not active')

        task_list = TaskList.objects.get(user=self.created_by)

        if order < 0 or order >= len(task_list.ordered_tasks):
            raise ValidationError('Order is out of bounds')

        task_list.ordered_tasks.remove(str(self.uid))
        task_list.ordered_tasks.insert(order, str(self.uid))
        task_list.save()

    def delete(self, *args, **kwargs):
        """
        update task order in task list when task is deleted from the database
        """
        task_list = TaskList.objects.filter(user=self.created_by).first()
        task_list.ordered_tasks.remove(str(self.uid))
        task_list.save()

        super(Task, self).delete(*args, **kwargs)

    class Meta:
        indexes = [
            models.Index(fields=['uid']),
            models.Index(fields=['start', 'end']),
            models.Index(fields=['due']),
            # models.Index(fields=['sequence']),
            models.Index(fields=['active']),
        ]

    def __str__(self):
        return "User %d: %s" % (self.created_by.id, self.summary)


class TaskList(Creatable):
    user = models.ForeignKey('auth.User', on_delete=models.CASCADE, unique=True)
    ordered_tasks = models.JSONField(default=list, blank=True)

    def __str__(self):
        return "User %d task list" % self.user.id


class TaskEvent(Creatable):

    class EventType(models.IntegerChoices):
        COMPLETED = 1, 'Completed'
        UNCOMPLETED = 2, 'Uncompleted'
        DELETED = 3, 'Deleted'

    task = models.ForeignKey(Task, on_delete=models.CASCADE)
    event_type = models.IntegerField(choices=EventType.choices)

    class Meta:
        indexes = [
            models.Index(fields=['task']),
            models.Index(fields=['event_type']),
        ]


@receiver(post_save, sender=User)
def create_user_data(sender, instance, created, **kwargs):
    if created:
        TaskList.objects.create(created_by=instance, user=instance)
