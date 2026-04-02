# Generate a Client for the InduFlow PartHub API

The InduFlow PartHub API provides a comprehensive OpenAPI Specification, enabling seamless client generation for a wide range of programming languages and frameworks. The specification details all available endpoints, including request structures, parameters, and response data schemas. This machine-readable format allows you to leverage widely available tools to generate robust, type-safe clients efficiently.

Note: The InduFlow NaviView application itself utilizes a generated client based on this specification.

---

## Overview of Client Generation

OpenAPI tools provide a streamlined approach to client generation. Among these, the OpenAPI Generator is a popular choice, offering support for numerous languages and frameworks. Running the generator as a Docker container eliminates the need for additional installation steps.

To get started:

- Determine your target programming language or framework  
- Verify its support in the official documentation overview  
- Ensure Docker is installed on your system  
- Download the OpenAPI Specification YAML for the Bosch Rexroth InduFlow PartHub API  
- Identify the appropriate generator name for your chosen language or framework from the generator list  

Execute the following command in the directory containing your `openApi.yaml` file, replacing `-g typescript-angular` with your desired generator:

```bash
docker run --rm -v %~dp0:/local openapitools/openapi-generator-cli generate -i /openApi.yaml -g typescript-angular -o /local/out --additional-properties basePath=https://induflow.boschrexroth.com