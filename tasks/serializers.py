from rest_framework import serializers

from tasks.models import Task


class TaskSerializer(serializers.ModelSerializer):
    class Meta:
        model = Task
        fields = ('uid', 'summary', 'description', 'created', 'completed', 'start', 'end', 'due', 'rrule', 'priority', 'urgency')


class PutTaskOrderSerializer(serializers.Serializer):
    order = serializers.IntegerField()
