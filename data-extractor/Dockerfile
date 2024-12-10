FROM ubuntu:noble-20241118.1

RUN apt-get update && apt-get -y upgrade && \
    apt-get -f -y install \
    curl \
    jq \
    python3 \
    python3-pika \
    && rm -rf /var/lib/apt/lists/*

# Install and configure Azure CLI
RUN curl -LsS https://aka.ms/InstallAzureCLIDeb | bash && rm -rf /var/lib/apt/lists/*
RUN az extension add --name azure-devops

COPY ./entrypoint.sh /app/entrypoint.sh
COPY ./queue.py /app/queue.py

ENTRYPOINT ["/app/entrypoint.sh"]
