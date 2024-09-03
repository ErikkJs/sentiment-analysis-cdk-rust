import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as iam from "aws-cdk-lib/aws-iam";

export class MedicalComprehendStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const comprehendFunction = new lambda.Function(this, "ComprehendMedicalFunction", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset("lambda/medical-entity-extractor/medical-entity-extractor.zip"),
      handler: "medical_comprehend",
    });

    comprehendFunction.addToRolePolicy(
      new iam.PolicyStatement({
        actions: ["comprehendmedical:DetectEntitiesV2"],
        resources: ["*"],
      })
    );

    const api = new apigateway.RestApi(this, "MedicalComprehendApi", {
      restApiName: "Medical Comprehend Service",
      description: "This service extracts medical entities from clinical text.",
    });

    const comprehendIntegration = new apigateway.LambdaIntegration(comprehendFunction);
    api.root.addMethod("POST", comprehendIntegration);
  }
}
