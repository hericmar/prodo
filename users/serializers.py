from datetime import datetime, timedelta

from rest_framework_simplejwt.serializers import TokenObtainPairSerializer


class ProdoTokenObtainPairSerializer(TokenObtainPairSerializer):
    @classmethod
    def get_token(cls, user):
        token = super().get_token(user)

        # TODO make timezone aware
        token['iat'] = datetime.now() - timedelta(hours=5)
        token['username'] = user.username
        token['fullname'] = f"{user.first_name} {user.last_name}"

        return token
