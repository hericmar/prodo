from rest_framework import status
from rest_framework.generics import DestroyAPIView, RetrieveAPIView, CreateAPIView, get_object_or_404
from rest_framework.permissions import IsAuthenticated
from rest_framework.response import Response

from base.serializers import EmptySerializer
from ical.models import Subscription
from ical.serializers import SubscriptionSerializer


class SubscriptionView(RetrieveAPIView, CreateAPIView, DestroyAPIView):
    serializer_class = SubscriptionSerializer
    permission_classes = [IsAuthenticated]

    def get_serializer_class(self):
        if self.request.method == 'POST':
            return EmptySerializer
        return SubscriptionSerializer

    def get_queryset(self):
        return Subscription.objects.filter(user=self.request.user)

    def get_object(self):
        """
        Used by RetrieveAPIView to retrieve the object for this view.
        """
        return get_object_or_404(self.get_queryset())

    def perform_create(self, serializer):
        if Subscription.objects.filter(user=self.request.user).exists():
            return Response(
                {"error": "subscription already exists"},
                status=status.HTTP_409_CONFLICT
            )

        Subscription.objects.create(
            created_by=self.request.user,
            user=self.request.user,
            secret=Subscription.generate_secret()
        )

    def perform_destroy(self, instance):
        if not Subscription.objects.filter(user=self.request.user).exists():
            return Response(
                {"error": "subscription does not exists"},
                status=status.HTTP_404_NOT_FOUND
            )

        Subscription.objects.filter(user=self.request.user).delete()
