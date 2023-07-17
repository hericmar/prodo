from django.urls import path

from tasks.views import TaskListView, TaskDetailView, put_order

urlpatterns = [
    path('', TaskListView.as_view()),
    path('/<uid>', TaskDetailView.as_view()),
    path('/<uid>/order', put_order),
]
