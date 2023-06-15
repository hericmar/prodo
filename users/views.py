from rest_framework_simplejwt.views import TokenObtainPairView

from users.serializers import ProdoTokenObtainPairSerializer


class ProdoTokenObtainPairView(TokenObtainPairView):
    serializer_class = ProdoTokenObtainPairSerializer
