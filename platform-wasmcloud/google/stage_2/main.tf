provider "google" {
  project = var.project_id
  region  = var.region
}

data "terraform_remote_state" "stage_1" {
  backend = "gcs"

  config = {
    bucket = "platform-poc-wasmcloud-tofu-state"
    prefix = "dev"
  }
}

provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = data.terraform_remote_state.stage_1.outputs.kubernetes_context
}
