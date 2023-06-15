from django.urls import path
from rest_framework_simplejwt.views import (
    TokenRefreshView, TokenBlacklistView,
)

from users.views import ProdoTokenObtainPairView

urlpatterns = [
    path('token', ProdoTokenObtainPairView.as_view(), name='token_obtain_pair'),
    path('token/refresh', TokenRefreshView.as_view(), name='token_refresh'),
    path('token/blacklist', TokenBlacklistView.as_view(), name='token_blacklist'),
]
