# Sentiment Analysis and Medical Entity Extraction with AWS Lambda and Rust

This weekend project contains two AWS Lambda functions implemented in Rust. We used AWS Comprehend for text sentiment analysis and AWS Comprehend Medical for extracting medical entities from text. The Lambda functions are deployed using AWS CDK and can be accessed via their respective api gateways. Yes I could have put them all in one api and stack but we ball. 

## Table of Contents

- [Overview](#overview)
- [Setting Up](#setting-up)
- [Building the Rust source files](#building-the-rust-executables)
  - [Sentiment Analysis Lambda](#sentiment-analysis-lambda)
  - [Medical Entity Extraction Lambda](#medical-entity-extraction-lambda)
- [Deploying](#deploying-the-cdk-stack)
- [Testing the APIs](#testing-the-apis)
- [Troubleshooting](#troubleshooting)

## Overview

This project provides two distinct API's:
- **Sentiment Analysis**: Uses AWS Comprehend to analyze the sentiment of provided text.
- **Medical Entity Extraction**: Utilizes AWS Comprehend Medical to extract medical-related entities from text.

Each Lambda function resides in its own directory under the `lambda` folder, with separate source files, and build scripts.

## Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Cross**: Install with `cargo install cross`
- **AWS CLI**: [Install AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and configure it with `aws configure`
- **AWS CDK**: Install with `npm install -g aws-cdk`
- **Docker**: Required by `cross` for cross-compilation (can be compiled with Cargo if on unix based machines)
- **Cargo Make**: Install with `cargo install cargo-make`

## Setting Up

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/your-username/sentiment_analysis_cdk_rust.git
   cd sentiment_analysis_cdk_rust
   ```

2. Bootstrap CDK Environment (if not already done):
bash
    ```bash
    npx cdk bootstrap
    ```
3. add your keys and enviroment variables: 
    **.env**
    ```
    AWS_ACCESS_KEY_ID=
    AWS_SECRET_ACCESS_KEY=
    AWS_REGION=us-east-1
    AWS_ACCOUNT_ID=
    ```
## Building the Rust executables

Each Lambda function is managed independently with its own `Makefile.toml` located in its respective directory.

### Sentiment Analysis Lambda

1. **Navigate to the sentiment analysis directory:**
   ```bash
   cd lambda/sentiment_analysis
   ```
2. **Build, Package, and Deploy:**
- Run with Cargo Make:
    ```bash
    cargo make
    ```
This command will:
- Compile the Rust code using cross for the correct target.
- Package the binary into a zip file.

### Medical Entity Extraction Lambda

1. **Navigate to the sentiment analysis directory:**
   ```bash
   cd lambda/medical-entity-extractor
   ```
2. **Build, Package, and Deploy:**
- Run with Cargo Make:
    ```bash
    cargo make
    ```
This command will:
- Compile the Rust code using cross for the correct target.
- Package the binary into a zip file.

## Deploying the CDK Stack
To deploy the CDK stack, run the following command from the root of the project:

```bash
cdk deploy
```


## Testing the APIs

After deployment, you can test each API by sending POST requests to the respective API Gateway endpoints provided in the CDK output or AWS console. Personally i like using the console since we can use a sample http event.

**Example using `curl` for Sentiment Analysis:**
```bash
curl -X POST <SENTIMENT_ANALYSIS_API_URL> -H "Content-Type: application/json" -d '{"text":"I love Rust programming!"}'
```

## Troubleshooting
- **Lambda Not Executing Properly:** Ensure the binary in the zip file is named bootstrap and is directly in the root of the zip file.

- **Permission Issues:** Verify that the Lambda execution role has the necessary permissions for AWS Comprehend and CloudWatch logging.

- **Build Failures:** Ensure Docker is running and accessible on your system for cross to function correctly.