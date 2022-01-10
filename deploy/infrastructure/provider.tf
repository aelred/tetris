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
  }
}

provider aws {
  region = "eu-west-2"
}

provider github {}

provider kubernetes {
  config_path = "~/.kube/config"
}