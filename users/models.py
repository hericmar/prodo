from django.contrib.auth.models import User
from django.db import models
from django.db.models.signals import post_save
from django.dispatch import receiver


class UserData(models.Model):
    user = models.OneToOneField('auth.User', on_delete=models.CASCADE)
    urgency_calculated_at = models.DateTimeField(null=True, blank=True)


@receiver(post_save, sender=User)
def create_user_data(sender, instance, created, **kwargs):
    if created:
        UserData.objects.create(user=instance)
