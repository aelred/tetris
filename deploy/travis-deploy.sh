  chmod 600 deploy_key
  mv deploy_key ~/.ssh/id_rsa
  ssh felix@ael.red "bash -s" < deploy/deploy.sh
