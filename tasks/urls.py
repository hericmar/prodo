from django.urls import path

from tasks.views import TaskListView, TaskDetailView

urlpatterns = [
    path('', TaskListView.as_view()),
    path('/<uid>', TaskDetailView.as_view()),
]
