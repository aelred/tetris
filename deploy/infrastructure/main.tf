module service {
  source = "github.com/aelred/provision//modules/service"
  name = "tetris"
  image = "tetris-server"
}

module s3_site {
  source = "github.com/aelred/provision//modules/s3_site"
  repository = "tetris"
  domain = "ael.red"
  subdomain = "tetris"
}

output next_steps {
  value = module.service.next_steps
}

output dockerhub_webhook_url {
  value = module.service.dockerhub_webhook_url
}
