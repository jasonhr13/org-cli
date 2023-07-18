# fuzzy-cli
Fuzzy CLI includes tools to make working at Fuzzy easier.  

Prerequisites for all commands:
- AWS CLI installed and configured
  - Visit: https://awscli.amazonaws.com/AWSCLIV2.pkg
  - Once installed run `aws configure` and input the credentials given to you
  - Set `us-west-2` as your default region

### Install/Upgrade fuzzy-cli
```
/bin/bash -c "$(curl -fsSL https://fuzzy-cli.s3.us-west-2.amazonaws.com/install.sh)"
```

### SSH <Server Name>
SSH into a specific server if it is running.
```
fuzzy-cli ssh Yankee
```

### SSH -p (-a/-b) <Server Name>
SSH directly into production server by name and availability zone.
```
fuzzy-cli ssh -p -a api-production
```
_note:_ Without the `-a` or `-b` flag you will get a list of servers matching the given name to pick from.

### List-Servers
Displays a list of all servers in AWS, selecting a server triggers an SSH into that server.
```
fuzzy-cli list-servers
```
