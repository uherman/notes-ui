# Description: Build and push the docker image and deploy the ui to the k3s cluster

# Build and push the docker image
docker build . -t uherman/notes-md-ui
docker push uherman/notes-md-ui

# Replace tokens in the deployment.yml file
export $(grep -v '^#' .env.secret | xargs)
envsubst < deploy/deployment.yml > deploy/.secret.deployment.yml

# Deploy the api to the k3s cluster
k3s apply -f deploy/.secret.deployment.yml
k3s rollout restart deployment notes-md-ui

# Clean up
rm deploy/.secret.deployment.yml