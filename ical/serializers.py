from rest_framework import serializers

from ical.models import Subscription


class SubscriptionSerializer(serializers.ModelSerializer):

    class Meta:
        model = Subscription
        fields = ('secret',)


class PostSubscriptionSerializer(serializers.Serializer):

    def to_representation(self, instance):
        return SubscriptionSerializer(instance).data
