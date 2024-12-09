FROM ubuntu:noble-20241118.1

# Install Redis CLI
RUN apt-get update && apt-get -y upgrade && \
    apt-get -f -y install curl jq redis-tools && \
    rm -rf /var/lib/apt/lists/*

# Install and configure Azure CLI
RUN curl -LsS https://aka.ms/InstallAzureCLIDeb | bash && rm -rf /var/lib/apt/lists/*
RUN az extension add --name azure-devops

COPY ./entrypoint.sh /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
