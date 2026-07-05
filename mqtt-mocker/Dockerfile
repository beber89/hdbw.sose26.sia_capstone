ARG AUTH_USERNAME="user"
ARG AUTH_PASSWORD="user"

FROM python:3.13-alpine
ENV AUTH_USERNAME=${AUTH_USERNAME}
ENV AUTH_PASSWORD=${AUTH_PASSWORD}
WORKDIR /usr/src/app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
RUN mkdir config/


COPY . .
RUN python render_settings.py 

ENTRYPOINT [ "python", "./mqtt-simulator/main.py"]
