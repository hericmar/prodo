from django.urls import path

from ical.views import SubscriptionView

urlpatterns = [
    path('', SubscriptionView.as_view()),
]
