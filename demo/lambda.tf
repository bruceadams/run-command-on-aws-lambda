locals {
  function_name = "demonstrate-run-command-on-aws-lambda"
}

resource "aws_lambda_function" "demonstrate-run-command-on-aws-lambda" {
  depends_on    = [null_resource.build-and-push-image]
  function_name = local.function_name
  image_uri     = local.image_uri
  package_type  = "Image"
  role          = aws_iam_role.role.arn
}

data "aws_iam_policy_document" "role_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
  }
}

resource "aws_iam_role" "role" {
  name                = local.function_name
  assume_role_policy  = data.aws_iam_policy_document.role_policy.json
  managed_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
}

data "aws_lambda_invocation" "inspect" {
  depends_on    = [aws_lambda_function.demonstrate-run-command-on-aws-lambda]
  function_name = local.function_name
  input = jsonencode({
    program = "sh"
    arguments = [
      "-cx",
      "id ; env | sort ; ls -al / ; ifconfig ; netstat -an"
    ]
  })
}
