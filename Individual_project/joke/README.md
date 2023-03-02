# Joke Microservice

This microservice is built with Rust and Actix-Web to retrieve random jokes from icanhazdadjoke API and display them on a web page.

## How to use

To run the microservice, you need to have Rust and Cargo installed on your machine.

1. Clone the repository
2. Run `cargo build` to build the project
3. Run  `cargo run` to start the microservice on http://localhost:8080/
Endpoints
/
The home page displays a random joke fetched from the icanhazdadjoke API.

/single
This endpoint returns a single joke in plain text format.

Deployment
Docker
To build and run the Docker image:

Install Docker on your machine.
Run docker build -t joke . to build the Docker image.
Run docker run -p 8080:8080 joke to start the Docker container on port 8080.
Kubernetes (Minikube)
To deploy the microservice to Minikube:

Install Minikube on your machine.
Run minikube start to start Minikube.
Run eval $(minikube docker-env) to use Minikube's Docker environment.
Run docker build -t joke . to build the Docker image in Minikube's environment.
Run kubectl apply -f k8s-deployment.yaml to deploy the microservice to Kubernetes.
AWS ECR & ECS
To deploy the microservice to AWS ECR and ECS:

Install the AWS CLI and configure your credentials.
Run aws ecr create-repository --repository-name joke to create a repository on AWS ECR.
Run aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <account_id>.dkr.ecr.<region>.amazonaws.com to authenticate Docker to your registry.
Run docker build -t joke . to build the Docker image.
Run docker tag joke:latest <account_id>.dkr.ecr.<region>.amazonaws.com/joke:latest to tag the Docker image for ECR.
Run docker push <account_id>.dkr.ecr.<region>.amazonaws.com/joke:latest to push the Docker image to ECR.
Run ecs-cli configure --cluster joke-cluster --default-launch-type FARGATE --region <region> to configure the ECS CLI.
Run ecs-cli up --cluster joke-cluster --ecs-profile default to create the ECS cluster.
Run ecs-cli compose service up --cluster joke-cluster to deploy the microservice to ECS.
Conclusion
This microservice is a simple demonstration of building a Rust microservice with Actix-Web and deploying it to various platforms. Feel free to modify and use it in your own projects!

@purmecia ➜ /workspaces/rust_new_world (main) $ minikube start
😄  minikube v1.29.0 on Debian 11.6 (docker/amd64)
✨  Automatically selected the docker driver. Other choices: none, ssh
📌  Using Docker driver with root privileges
👍  Starting control plane node minikube in cluster minikube
🚜  Pulling base image ...
💾  Downloading Kubernetes v1.26.1 preload ...
    > preloaded-images-k8s-v18-v1...:  397.05 MiB / 397.05 MiB  100.00% 102.59 
    > gcr.io/k8s-minikube/kicbase...:  407.19 MiB / 407.19 MiB  100.00% 28.30 M
🔥  Creating docker container (CPUs=2, Memory=2200MB) ...
🐳  Preparing Kubernetes v1.26.1 on Docker 20.10.23 ...
    ▪ Generating certificates and keys ...
    ▪ Booting up control plane ...
    ▪ Configuring RBAC rules ...
🔗  Configuring bridge CNI (Container Networking Interface) ...
    ▪ Using image gcr.io/k8s-minikube/storage-provisioner:v5
🔎  Verifying Kubernetes components...
🌟  Enabled addons: storage-provisioner, default-storageclass
🏄  Done! kubectl is now configured to use "minikube" cluster and "default" namespace by default
@purmecia ➜ /workspaces/rust_new_world (main) $ kubectl create deployment joke-node --image=joke
deployment.apps/joke-node created
@purmecia ➜ /workspaces/rust_new_world (main) $ kubectl get deployments
NAME        READY   UP-TO-DATE   AVAILABLE   AGE
joke-node   0/1     1            0           9s
@purmecia ➜ /workspaces/rust_new_world (main) $ kubectl get pods
NAME                         READY   STATUS             RESTARTS   AGE
joke-node-864bb4d89c-zzlx5   0/1     ImagePullBackOff   0          15s

@purmecia ➜ /workspaces/rust_new_world (main) $ minikube dashboard --url
🔌  Enabling dashboard ...
    ▪ Using image docker.io/kubernetesui/metrics-scraper:v1.0.8
    ▪ Using image docker.io/kubernetesui/dashboard:v2.7.0
💡  Some dashboard features require the metrics-server addon. To enable all features please run:

        minikube addons enable metrics-server


🤔  Verifying dashboard health ...
🚀  Launching proxy ...
🤔  Verifying proxy health ...
http://127.0.0.1:43863/api/v1/namespaces/kubernetes-dashboard/services/http:kubernetes-dashboard:/proxy/

@purmecia ➜ /workspaces/rust_new_world (main) $ kubectl expose deployment joke-node --type=LoadBalancer --port=8080
service/joke-node exposed