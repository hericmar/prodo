from django.urls import path

from ical.views import SubscriptionView, ical_view

urlpatterns = [
    path('', SubscriptionView.as_view()),
    path('/<str:secret>', ical_view),
]
