from datetime import datetime

from rest_framework_simplejwt.serializers import TokenObtainPairSerializer


class ProdoTokenObtainPairSerializer(TokenObtainPairSerializer):
    @classmethod
    def get_token(cls, user):
        token = super().get_token(user)

        # Add custom claims
        token['iat'] = datetime.now()
        token['username'] = user.username
        token['fullname'] = f"{user.first_name} {user.last_name}"

        return token
