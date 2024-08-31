import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as iam from "aws-cdk-lib/aws-iam"; // Import IAM module

export class SentimentAnalysisCdkRustStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const sentimentAnalysisFunction = new lambda.Function(
      this,
      "SentimentAnalysisFunction",
      {
        runtime: lambda.Runtime.PROVIDED_AL2, 
        code: lambda.Code.fromAsset(
          "lambda/sentiment_analysis/sentiment_analysis.zip"
        ),
        handler: "sentiment_analysis",
      }
    );

    sentimentAnalysisFunction.addToRolePolicy(
      new iam.PolicyStatement({
        actions: ["comprehend:DetectSentiment"],
        resources: ["*"],
      })
    );

    const api = new apigateway.RestApi(this, "sentiment-analysis-api", {
      restApiName: "Sentiment Analysis Service",
      description: "This service analyzes text for sentiment.",
    });

    const sentimentIntegration = new apigateway.LambdaIntegration(
      sentimentAnalysisFunction
    );

    api.root.addMethod("POST", sentimentIntegration);
  }
}
