#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { SentimentAnalysisCdkRustStack } from "../lib/sentiment_analysis_cdk_rust-stack";
import { MedicalComprehendStack } from "../lib/medical_comprehend_cdk_rust-stack";

const app = new cdk.App();
new SentimentAnalysisCdkRustStack(app, "SentimentAnalysisCdkRustStack", {
  env: { account: "456103903859", region: "us-east-1" },
});

new MedicalComprehendStack(app, "MedicalComprehendStack", {
  env: { account: "456103903859", region: "us-east-1" },
});
