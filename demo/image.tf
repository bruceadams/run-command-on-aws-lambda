resource "aws_ecr_repository" "demonstrate-run-command-on-aws-lambda" {
  name = "demonstrate-run-command-on-aws-lambda"
}

locals {
  ecr_url   = aws_ecr_repository.demonstrate-run-command-on-aws-lambda.repository_url
  image_uri = "${local.ecr_url}:0.2.0"
}

resource "null_resource" "build-and-push-image" {
  triggers = { resource_arn = local.image_uri }

  provisioner "local-exec" {
    command = <<-EOT
      docker build . --tag ${local.image_uri}

      aws ecr get-login-password \
        | docker login --username AWS --password-stdin ${local.ecr_url}

      docker push ${local.image_uri}
    EOT
  }
}
