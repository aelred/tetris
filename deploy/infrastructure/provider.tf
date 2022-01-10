terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "~> 3.28.0"
    }
    github = {
      source = "integrations/github"
      version = "~> 4.4.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0.2"
    }
    kubectl = {
      source  = "gavinbunney/kubectl"
      version = "~> 1.10.0"
    }
  }
}

provider aws {
  region = "eu-west-2"
}

provider github {}

provider kubectl {}

provider kubernetes {
  config_path = "~/.kube/config"
}