import secrets

from django.db import models

from base.models import Creatable


SECRET_LENGTH = 64


class Subscription(Creatable):
    """
    A subscription to tasks in iCalendar format
    """
    user = models.ForeignKey('auth.User', on_delete=models.CASCADE)
    secret = models.CharField(max_length=SECRET_LENGTH, editable=False)

    @staticmethod
    def generate_secret():
        return secrets.token_hex(SECRET_LENGTH)[0:SECRET_LENGTH]

    class Meta:
        indexes = [
            models.Index(fields=['user']),
            models.Index(fields=['secret'])
        ]
