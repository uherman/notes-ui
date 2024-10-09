# Description: Build and push the docker image and deploy the proxy service to the k3s cluster

dotnet publish Proxy/Proxy.csproj -c Release

# Build and push the docker image
docker build . -t uherman/notes-md-proxy
docker push uherman/notes-md-proxy

# Replace tokens in the deployment.yml file
export $(grep -v '^#' .env | xargs)
envsubst < deployment.yml > .secret.deployment.yml

# Deploy the proxy service to the k3s cluster
k3s apply -f .secret.deployment.yml
k3s rollout restart deployment notes-md-proxy

# Clean up
rm .secret.deployment.yml