import uuid

from dateutil import rrule
from django.db import models
from django.core.exceptions import ValidationError

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


class Task(Creatable, Updatable):
    """
    Partially derived from RFC 5545
    """
    uid = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    summary = models.CharField(max_length=40)
    description = models.TextField(default="", blank=True)
    start = models.DateTimeField(null=True, blank=True)
    end = models.DateTimeField(null=True, blank=True)
    due = models.DateTimeField(null=True, blank=True)
    rrule = models.CharField(max_length=255, validators=[validate_rrule], null=True, blank=True)
    sequence = models.PositiveIntegerField(default=0)
    active = models.BooleanField(default=True)
    completed = models.DateTimeField(null=True, blank=True)

    class Meta:
        indexes = [
            models.Index(fields=['uid']),
            models.Index(fields=['start', 'end']),
            models.Index(fields=['due']),
            # models.Index(fields=['sequence']),
            models.Index(fields=['active']),
        ]

    def save(self, *args, **kwargs):
        """
        update sequence number when task is updated in the database
        """
        try:
            previous = Task.objects.get(uid=self.uid)

            if not previous.active:
                # TODO check if task is not already inactive, must be done in the database
                raise ValidationError('Task is not active')

            self.sequence += 1
        except Task.DoesNotExist:
            pass

        super(Task, self).save(*args, **kwargs)

    def __str__(self):
        return "User %d: %s" % (self.created_by.id, self.summary)


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


class Subscription(models.Model):
    """
    A subscription to a task
    """
    user = models.ForeignKey('auth.User', on_delete=models.CASCADE)
    secret = models.CharField(max_length=64, editable=False)

    class Meta:
        indexes = [
            models.Index(fields=['user']),
            models.Index(fields=['secret'])
        ]
