# Run command on AWS Lambda

[![Build Status](https://api.cirrus-ci.com/github/bruceadams/run-command-on-aws-lambda.svg)](https://cirrus-ci.com/github/bruceadams/run-command-on-aws-lambda)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Apache License](https://img.shields.io/github/license/bruceadams/run-command-on-aws-lambda?logo=apache)](LICENSE)
[![Github Release](https://img.shields.io/github/v/release/bruceadams/run-command-on-aws-lambda?logo=github)](https://github.com/bruceadams/run-command-on-aws-lambda/releases)
[![Crates.io](https://img.shields.io/crates/v/run-command-on-aws-lambda?logo=rust)](https://crates.io/crates/run-command-on-aws-lambda)

## Goal

Make it easy to run a command line in a Docker image on AWS Lambda.

This will work with many different Docker images, from plain Linux
distributions, to AWS Command Line tools, to large standalone tools like
[SchemaSpy](https://schemaspy.org/).

## Some example runs on an Alpine Linux base image

### Who is the runtime user? `id`

With a test event of `{"program": "id"}` the result looks like this:

```
START RequestId: 955381a3-859a-4bf0-b560-acd0e76d6015 Version: $LATEST
Running id [].
Waiting for id completion.
uid=993(sbx_user1051) gid=990
Child id exited successfully.
END RequestId: 955381a3-859a-4bf0-b560-acd0e76d6015
REPORT RequestId: 955381a3-859a-4bf0-b560-acd0e76d6015	Duration: 1.29 ms	Billed Duration: 2 ms	Memory Size: 128 MB	Max Memory Used: 13 MB
```

Even though the default runtime user for the image is `root`,
AWS Lambda runs the image as a non-privileged user.
Nice security work by the AWS folks!

### What are the filesystems? `mount`

With a test event of `{"program": "mount"}` the result looks like this:

```
START RequestId: 2602928d-b146-44f6-b684-bfe95e8b0714 Version: $LATEST
Running mount [].
Waiting for mount completion.
/mnt/root-rw/opt/amazon/asc/worker/tasks/rtfs/inline-manifest on / type overlay (ro,nosuid,nodev,relatime,lowerdir=/tmp/es2088019054/8c01de2338356980:/tmp/es2088019054/34f08a2940031490)
/dev/vdb on /dev type ext4 (rw,nosuid,noexec,noatime,data=writeback)
/dev/vdd on /tmp type ext4 (rw,relatime,data=writeback)
none on /proc type proc (rw,nosuid,nodev,noexec,noatime)
/dev/vdb on /proc/sys/kernel/random/boot_id type ext4 (ro,nosuid,nodev,noatime,data=writeback)
/dev/root on /etc/passwd type ext4 (ro,nosuid,nodev,relatime,data=ordered)
/dev/root on /var/rapid type ext4 (ro,nosuid,nodev,relatime,data=ordered)
/dev/vdb on /etc/resolv.conf type ext4 (ro,nosuid,nodev,noatime,data=writeback)
Child mount exited successfully.
END RequestId: 2602928d-b146-44f6-b684-bfe95e8b0714
REPORT RequestId: 2602928d-b146-44f6-b684-bfe95e8b0714	Duration: 1.44 ms	Billed Duration: 2 ms	Memory Size: 128 MB	Max Memory Used: 13 MB
```

Notice that the root filesystem (the `/` mount point) is read-only.
Yet more good security work by the AWS folks, but this can be a
surprise for a runtime that expects to be able to write to disk.

### How much space is available on `/tmp`?

In the file system list above, we can see that `/tmp` is writable.
Now the question is: how much space is available? With a test event of:

```json
{
  "program": "df",
  "arguments": ["-h", "/tmp"]
}
```

the result is:

```
START RequestId: 59a5bbb8-2ec6-42a3-8c5f-1b4a1fac476f Version: $LATEST
Running df ["-h", "/tmp"].
Waiting for df completion.
Filesystem                Size      Used Available Use% Mounted on
/dev/vdd                525.8M    872.0K    513.4M   0% /tmp
Child df exited successfully.
END RequestId: 59a5bbb8-2ec6-42a3-8c5f-1b4a1fac476f
REPORT RequestId: 59a5bbb8-2ec6-42a3-8c5f-1b4a1fac476f	Duration: 1.57 ms	Billed Duration: 2 ms	Memory Size: 128 MB	Max Memory Used: 12 MB
```

We can see about a half gigabyte of available space on `/tmp`.

## Origin story

I worked with a software partner who provides a service implemented in
[Microsoft .NET](https://dotnet.microsoft.com/). Part of the long term
lifecycle of this service includes database migrations. We agreed with
our partner to handle migrations separately from the main service.
(We expect to run a cluster of servers for the main service.
We would like to avoid confusing database migration races
when first starting, or upgrading, the cluster.)

The partner provides the service packaged in Docker images.
One of these images does the database migrations.
This project gives me a handy way to run the database migrations in AWS
and to know when those migrations are complete or if something failed.

### Wrap the partner image

We create a new Docker image that wraps the partner provided image
adding `run-command-on-aws-lambda` as the entrypoint.

```dockerfile
FROM partner/dotnet-service-with-migrations:1.0

ARG WRAPPER=https://github.com/bruceadams/run-command-on-aws-lambda/releases/download/v0.1.0/run-command-on-aws-lambda.linux.x86_64
ADD ${WRAPPER} /usr/local/bin/run-command-on-aws-lambda
RUN chmod +x /usr/local/bin/run-command-on-aws-lambda

ENTRYPOINT [ "run-command-on-aws-lambda" ]
```

### Run the migrations

We deploy this image as an AWS Lambda,
including AWS VPC setups so we can reach the database.
We then invoke the Lambda providing what it.

In our Terraform configurations, we invoke the Lambda providing it
with database connection information via environment variables.

```hcl
data "aws_lambda_invocation" "example" {
  depends_on    = [aws_lambda_function.migration]
  function_name = local.function_name
  input = jsonencode({
    program     = "dotnet"
    arguments   = ["DatabaseMigrations.dll"]
    environment = var.database_environment
  })
}
```

Once this Lambda invocation finishes, we proceed to setup or upgrade
the service cluster. If the Lambda invocation fails,
we make no changes to the cluster.
