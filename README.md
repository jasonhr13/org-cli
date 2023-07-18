# org-cli
Simple CLI for accessing EC2 instances on AWS.

Prerequisites for all commands:
- AWS CLI installed and configured
  - Visit: https://awscli.amazonaws.com/AWSCLIV2.pkg
  - Once installed run `aws configure` and input the credentials given to you
  - Set `us-west-2` as your default region

### Install/Upgrade org-cli
```
/bin/bash -c "$(curl -fsSL S3-LOCATION/install.sh)"
```

### SSH <Server Name>
SSH into a specific server if it is running.
```
org-cli ssh staging
```

### SSH -p (-a/-b) <Server Name>
SSH directly into production server by name and availability zone.
```
org-cli ssh -p -a production
```
_note:_ Without the `-a` or `-b` flag you will get a list of servers matching the given name to pick from.

### List-Servers
Displays a list of all servers in AWS, selecting a server triggers an SSH into that server.
```
org-cli list-servers
```
