from django.http import HttpResponse
from rest_framework.generics import CreateAPIView, ListAPIView, RetrieveAPIView, UpdateAPIView, DestroyAPIView
from rest_framework.permissions import IsAuthenticated

from tasks.models import Task
from tasks.serializers import TaskSerializer


class TaskListView(ListAPIView, CreateAPIView):
    serializer_class = TaskSerializer
    permission_classes = [IsAuthenticated]

    def get_queryset(self):
        """
        This view should return a list of all the active tasks
        for the currently authenticated user.
        """
        return Task.objects.filter(created_by=self.request.user, active=True)

    def perform_create(self, serializer):
        serializer.save(created_by=self.request.user)


class TaskDetailView(RetrieveAPIView, DestroyAPIView, UpdateAPIView):
    serializer_class = TaskSerializer
    permission_classes = [IsAuthenticated]
    lookup_field = 'uid'

    def get_queryset(self):
        return Task.objects.filter(created_by=self.request.user)

    def perform_update(self, serializer):
        serializer.save(updated_by=self.request.user)

    def perform_destroy(self, instance):
        instance.active = False
        instance.save()
