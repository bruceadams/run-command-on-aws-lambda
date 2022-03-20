# Demonstration for `run-command-on-aws-lambda`

This is a runnable demonstration of using
[run-command-on-aws-lambda](https://github.com/bruceadams/run-command-on-aws-lambda).

## Requirements

Running this requires that you have three command line tools installed.

- [AWS Command Line Interface](https://docs.aws.amazon.com/cli/)
- [Docker](https://www.docker.com/)
- [Terraform](https://www.terraform.io/)

and AWS credentials setup for command line use. Running `aws configure`
is one way to setup AWS credentials for command line use.

## Run

These two commands will run this demonstration. First, initialize Terraform.

```sh
terraform init
```

Then apply the Terraform configurations in the demo.

```sh
terraform apply
```

The Terraform apply

- creates an [AWS Elastic Container Registry](https://aws.amazon.com/ecr/) (AWS ECR)
- builds a Docker image of alpine with `run-command-on-aws-lambda`
- pushes the Docker image to AWS ECR
- creates an [AWS Lambda](https://aws.amazon.com/lambda/) to run the image
- invokes the Lambda to run a script to report on the Lambda environment

## Result

The results from the run appear in
[AWS CloudWatch Logs](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/).
The log group is named `/aws/lambda/demonstrate-run-command-on-aws-lambda`.

```
@timestamp    @message
18:12:18.879  START RequestId: 4943fbd8-67fd-4592-a7cb-731a1030a0b4 Version: $LATEST
18:12:18.880  Running sh ["-cx", "id ; env | sort ; ls -al / ; ifconfig ; netstat -an"].
18:12:18.881  Waiting for sh completion.
18:12:18.891  + id
18:12:18.892  uid=993(sbx_user1051) gid=990
18:12:18.910  + sort
18:12:18.910  + env
18:12:18.910  AWS_ACCESS_KEY_ID=XXX
18:12:18.910  AWS_DEFAULT_REGION=us-east-2
18:12:18.910  AWS_EXECUTION_ENV=AWS_Lambda_Image
18:12:18.910  AWS_LAMBDA_FUNCTION_MEMORY_SIZE=128
18:12:18.910  AWS_LAMBDA_FUNCTION_NAME=demonstrate-run-command-on-aws-lambda
18:12:18.910  AWS_LAMBDA_FUNCTION_VERSION=$LATEST
18:12:18.910  AWS_LAMBDA_INITIALIZATION_TYPE=on-demand
18:12:18.910  AWS_LAMBDA_LOG_GROUP_NAME=/aws/lambda/demonstrate-run-command-on-aws-lambda
18:12:18.910  AWS_LAMBDA_LOG_STREAM_NAME=2022/03/20/[$LATEST]52f987380ee74d3fa8deb8286ca32de0
18:12:18.910  AWS_LAMBDA_RUNTIME_API=127.0.0.1:9001
18:12:18.910  AWS_REGION=us-east-2
18:12:18.910  AWS_SECRET_ACCESS_KEY=XXXXXX
18:12:18.910  AWS_SESSION_TOKEN=base64-encoded-string
18:12:18.910  AWS_XRAY_CONTEXT_MISSING=LOG_ERROR
18:12:18.910  AWS_XRAY_DAEMON_ADDRESS=169.254.79.129:2000
18:12:18.910  LAMBDA_RUNTIME_DIR=/var/runtime
18:12:18.910  LAMBDA_TASK_ROOT=/var/task
18:12:18.910  PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
18:12:18.910  PWD=/
18:12:18.910  SHLVL=1
18:12:18.910  _AWS_XRAY_DAEMON_ADDRESS=169.254.79.129
18:12:18.910  _AWS_XRAY_DAEMON_PORT=2000
18:12:18.910  _HANDLER=
18:12:18.910  _X_AMZN_TRACE_ID=Root=1-62376e82-023560591c22fed61fcb84a4;Parent=734f596b7038e5e9;Sampled=0
18:12:18.929  + ls -al /
18:12:18.930  total 33
18:12:18.931  drwxr-xr-x    1 root     root          4096 Mar 20 18:03 .
18:12:18.931  drwxr-xr-x    1 root     root          4096 Mar 20 18:03 ..
18:12:18.931  drwxr-xr-x    2 root     root          2048 Mar 16 20:15 bin
18:12:18.931  drwxr-xr-x    2 root     root          4096 Mar 20 18:03 dev
18:12:18.931  drwxr-xr-x    1 root     root          4096 Mar 20 18:03 etc
18:12:18.931  drwxr-xr-x    2 root     root            60 Mar 16 20:15 home
18:12:18.931  drwxr-xr-x    7 root     root          1024 Mar 16 20:15 lib
18:12:18.931  drwxr-xr-x    5 root     root            60 Mar 16 20:15 media
18:12:18.931  drwxr-xr-x    1 root     root          4096 Mar 20 18:03 mnt
18:12:18.931  drwxr-xr-x    2 root     root            60 Mar 16 20:15 opt
18:12:18.931  dr-xr-xr-x   72 root     root             0 Mar 20 18:12 proc
18:12:18.931  drwx------    2 root     root            60 Mar 16 20:15 root
18:12:18.931  drwxr-xr-x    2 root     root            60 Mar 16 20:15 run
18:12:18.931  drwxr-xr-x    2 root     root          1024 Mar 16 20:15 sbin
18:12:18.931  drwxr-xr-x    2 root     root            60 Mar 16 20:15 srv
18:12:18.931  drwxr-xr-x    2 root     root            60 Mar 16 20:15 sys
18:12:18.931  drwx------    2 sbx_user 990           4096 Mar 20 18:12 tmp
18:12:18.931  drwxr-xr-x    7 root     root          1024 Mar 16 20:15 usr
18:12:18.931  drwxr-xr-x    1 root     root          4096 Mar 20 18:03 var
18:12:18.931  + ifconfig
18:12:18.950  lo        Link encap:Local Loopback
18:12:18.950            inet addr:127.0.0.1  Mask:255.0.0.0
18:12:18.950            UP LOOPBACK RUNNING  MTU:65536  Metric:1
18:12:18.950            RX packets:7 errors:0 dropped:0 overruns:0 frame:0
18:12:18.950            TX packets:7 errors:0 dropped:0 overruns:0 carrier:0
18:12:18.950            collisions:0 txqueuelen:1000
18:12:18.950            RX bytes:1031 (1.0 KiB)  TX bytes:1031 (1.0 KiB)
18:12:18.950  telemetry1_sb Link encap:Ethernet  HWaddr DE:F1:B5:70:86:65
18:12:18.950                inet addr:169.254.79.130  Bcast:0.0.0.0  Mask:255.255.255.252
18:12:18.950                UP BROADCAST RUNNING MULTICAST  MTU:1500  Metric:1
18:12:18.950                RX packets:0 errors:0 dropped:0 overruns:0 frame:0
18:12:18.950                TX packets:1 errors:0 dropped:0 overruns:0 carrier:0
18:12:18.950                collisions:0 txqueuelen:1000
18:12:18.950                RX bytes:0 (0.0 B)  TX bytes:90 (90.0 B)
18:12:18.950  vinternal_1 Link encap:Ethernet  HWaddr A2:A9:F9:54:75:60
18:12:18.950              inet addr:169.254.76.1  Bcast:0.0.0.0  Mask:255.255.254.0
18:12:18.950              UP BROADCAST RUNNING MULTICAST  MTU:1500  Metric:1
18:12:18.950              RX packets:0 errors:0 dropped:0 overruns:0 frame:0
18:12:18.950              TX packets:2 errors:0 dropped:0 overruns:0 carrier:0
18:12:18.950              collisions:0 txqueuelen:1000
18:12:18.950              RX bytes:0 (0.0 B)  TX bytes:176 (176.0 B)
18:12:18.950  + netstat -an
18:12:18.951  Active Internet connections (servers and established)
18:12:18.951  Proto Recv-Q Send-Q Local Address           Foreign Address         State
18:12:18.951  tcp        0      0 127.0.0.1:9001          0.0.0.0:*               LISTEN
18:12:18.951  tcp        0      0 127.0.0.1:57288         127.0.0.1:9001          ESTABLISHED
18:12:18.951  tcp        0      0 127.0.0.1:9001          127.0.0.1:57288         ESTABLISHED
18:12:18.951  udp        0      0 169.254.79.130:60764    169.254.79.129:2000     ESTABLISHED
18:12:18.951  Active UNIX domain sockets (servers and established)
18:12:18.951  Proto RefCnt Flags       Type       State         I-Node Path
18:12:18.970  Child sh exited successfully.
18:12:18.971  END RequestId: 4943fbd8-67fd-4592-a7cb-731a1030a0b4
18:12:18.971  REPORT RequestId: 4943fbd8-67fd-4592-a7cb-731a1030a0b4
                     Duration: 91.21 ms
                     Billed Duration: 236 ms
                     Memory Size: 128 MB
                     Max Memory Used: 13 MB
                     Init Duration: 143.90 ms
```

## Cleanup

One of the beauties of using Terraform is it can destroy what it created.

```sh
terraform destroy
```

The one item created by our run above that Terraform didn't directly create
and therefore does not destroy is the CloudWatch log group
`/aws/lambda/demonstrate-run-command-on-aws-lambda`.
You can delete this log group with the following AWS CLI command.

```sh
aws logs delete-log-group --log-group-name /aws/lambda/demonstrate-run-command-on-aws-lambda
```

## Notes

I am not aiming to provide an example of good Terraform practice in this
demonstration.
My primary goal is for it to be runnable _as is_, to keep the terraform
configurations short and to use long names for AWS resources.
I use long names in the hope that someone running this will be able
to recognize what was created by this demonstration in their AWS console.
