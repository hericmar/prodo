from django.db import models


class Creatable(models.Model):
    """
    Abstract base class for models that have a created_by and created timestamp.
    """
    created_by = models.ForeignKey('auth.User', related_name='%(class)s_created', on_delete=models.RESTRICT)
    created = models.DateTimeField(auto_now_add=True)

    class Meta:
        abstract = True


class Updatable(models.Model):
    """
    Abstract base class for models that have an updated_by and updated timestamp.
    """
    updated_by = models.ForeignKey(
        'auth.User',
        related_name='%(class)s_updated',
        on_delete=models.RESTRICT,
        null=True,
        blank=True
    )
    updated = models.DateTimeField(auto_now=True)

    class Meta:
        abstract = True
