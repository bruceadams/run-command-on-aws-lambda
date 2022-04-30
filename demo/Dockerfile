FROM --platform=linux/amd64 alpine

# Install the wrapper to run arbitrary commands in a Lambda.
ARG WRAPPER=https://github.com/bruceadams/run-command-on-aws-lambda/releases/download/v1.1.0/run-command-on-aws-lambda.linux.x86_64
ADD ${WRAPPER} /usr/local/bin/run-command-on-aws-lambda
RUN chmod +x /usr/local/bin/run-command-on-aws-lambda

ENTRYPOINT [ "run-command-on-aws-lambda" ]
