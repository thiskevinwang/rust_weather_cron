import * as events from "@aws-cdk/aws-events";
import * as dynamodb from "@aws-cdk/aws-dynamodb";
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
      ],
    });

    const lambdaFn = new lambda.Function(this, "Singleton", {
      code: lambda.Code.fromAsset("rust.zip", {}),
      /** @TODO — where does this handler name come from? */
      handler: "hello.handler",
      timeout: cdk.Duration.seconds(10),
      runtime: lambda.Runtime.PROVIDED_AL2,
      environment: {
        API_KEY: process.env.API_KEY!,
        TABLE_NAME: process.env.TABLE_NAME!,
      },
      role: customRole,
    });

    const table = dynamodb.Table.fromTableName(
      this,
      "Table",
      process.env.TABLE_NAME!
    );
    table.grantReadWriteData(lambdaFn);

    /**
     * Only create table once. aws-cdk currently doesn't support idempotent
     * executions with dynamoDb IaC.
     */
    // const table = new dynamodb.Table(this, "Table", {
    //   tableName: process.env.TABLE_NAME,
    //   partitionKey: { name: "PK", type: dynamodb.AttributeType.STRING },
    //   sortKey: { name: "SK", type: dynamodb.AttributeType.STRING },
    //   billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
    // });

    // table.addGlobalSecondaryIndex({
    //   indexName: "GSI_SK_PK",
    //   partitionKey: { name: "SK", type: dynamodb.AttributeType.STRING },
    //   sortKey: { name: "PK", type: dynamodb.AttributeType.STRING },
    // });
    // table.grantReadWriteData(lambdaFn);

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
