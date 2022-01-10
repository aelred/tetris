provider aws {
  region = "eu-west-2"
}

provider github {}

provider kubernetes {
  config_path = "~/.kube/config"
}