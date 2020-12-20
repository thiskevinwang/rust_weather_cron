import * as events from "@aws-cdk/aws-events";
import * as targets from "@aws-cdk/aws-events-targets";
import * as lambda from "@aws-cdk/aws-lambda";
import * as iam from "@aws-cdk/aws-iam";
import * as cdk from "@aws-cdk/core";
import * as dotenv from "dotenv";

/**
 * Enable CDK to pick up environment variables in `.env`
 * This will be added to the LambdaFn's env, and read by
 * the rust binary.
 */
dotenv.config();

export class LambdaCronStack extends cdk.Stack {
  constructor(app: cdk.App, id: string) {
    super(app, id);

    const customRole = new iam.Role(this, `${id}-customRole`, {
      roleName: `${id}-customRole`,
      assumedBy: new iam.ServicePrincipal("lambda.amazonaws.com"),
      managedPolicies: [
        // aws-managed
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AWSLambdaBasicExecutionRole"
        ),
        // customer-managed
        iam.ManagedPolicy.fromManagedPolicyName(
          this,
          "DynamoReadAndWrite",
          "DynamoReadAndWrite"
        ),
      ],
    });
    const lambdaFn = new lambda.Function(this, "Singleton", {
      code: lambda.Code.fromAsset("rust.zip", {}),
      handler: "hello.handler",
      timeout: cdk.Duration.seconds(10),
      runtime: lambda.Runtime.PROVIDED_AL2,
      environment: {
        API_KEY: process.env.API_KEY!,
      },
      role: customRole,
    });

    /**
     * Our cron rule — _"Every hour"_
     * - See https://docs.aws.amazon.com/lambda/latest/dg/tutorial-scheduled-events-schedule-expressions.html
     */
    const rule = new events.Rule(this, "Rule", {
      schedule: events.Schedule.expression("cron(0 * ? * SUN-SAT *)"),
    });

    rule.addTarget(new targets.LambdaFunction(lambdaFn));
  }
}

const app = new cdk.App();
new LambdaCronStack(app, "LambdaCronExample");
app.synth();
