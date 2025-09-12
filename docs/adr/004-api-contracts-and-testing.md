# ADR-004: API Contracts and Testing Strategy

## Status
**ACCEPTED** - 2024-09-08

## Context
As a data engineering consultancy, we build numerous APIs for data processing, analytics, and system integration. These APIs must be reliable, well-documented, and thoroughly tested to ensure they meet client requirements and maintain high quality standards.

## Decision
We implement **comprehensive API contracts and testing** to ensure all APIs are robust, well-documented, and thoroughly validated.

### API Contract Strategy

#### Contract-First Development
- **API-first design**: Define contracts before implementation
- **Version management**: Maintain backward compatibility and versioning
- **Documentation**: Comprehensive API documentation with examples
- **Validation**: Runtime contract validation and enforcement

#### Contract Standards
- **OpenAPI 3.0**: Standard specification for REST APIs
- **AsyncAPI**: Specification for event-driven APIs
- **GraphQL Schema**: For GraphQL APIs
- **gRPC Proto**: For gRPC services
- **WebSocket Schema**: For real-time APIs

## Implementation

### API Contract Definition
```yaml
# Example: OpenAPI 3.0 contract for data processing API
openapi: 3.0.3
info:
  title: Data Processing API
  description: High-performance data processing API built with Rust
  version: 1.0.0
  contact:
    name: Data Engineering Pro
    email: contact@dataengineeringpro.com

servers:
  - url: https://api.dataengineeringpro.com/v1
    description: Production server
  - url: https://staging-api.dataengineeringpro.com/v1
    description: Staging server

paths:
  /data/process:
    post:
      summary: Process data pipeline
      description: Submit data for processing through our Rust-powered pipeline
      operationId: processData
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ProcessRequest'
            examples:
              csv_processing:
                summary: CSV Processing Example
                value:
                  data_type: "csv"
                  source: "https://example.com/data.csv"
                  processor: "rust-processor"
                  output_format: "json"
                  options:
                    delimiter: ","
                    has_header: true
      responses:
        '200':
          description: Processing successful
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ProcessResponse'
        '400':
          description: Bad request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '500':
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

components:
  schemas:
    ProcessRequest:
      type: object
      required:
        - data_type
        - source
        - processor
      properties:
        data_type:
          type: string
          enum: [csv, json, parquet, avro]
          description: Type of data to process
        source:
          type: string
          format: uri
          description: Source of the data
        processor:
          type: string
          enum: [rust-processor, python-processor, custom-processor]
          description: Processing engine to use
        output_format:
          type: string
          enum: [json, csv, parquet, avro]
          default: json
          description: Output format
        options:
          type: object
          description: Processing options
          properties:
            delimiter:
              type: string
              default: ","
            has_header:
              type: boolean
              default: true
            compression:
              type: string
              enum: [none, gzip, lz4, zstd]
              default: none

    ProcessResponse:
      type: object
      required:
        - job_id
        - status
        - created_at
      properties:
        job_id:
          type: string
          format: uuid
          description: Unique job identifier
        status:
          type: string
          enum: [queued, processing, completed, failed]
          description: Current job status
        created_at:
          type: string
          format: date-time
          description: Job creation timestamp
        estimated_completion:
          type: string
          format: date-time
          description: Estimated completion time
        result_url:
          type: string
          format: uri
          description: URL to download results (when completed)

    ErrorResponse:
      type: object
      required:
        - error
        - message
        - timestamp
      properties:
        error:
          type: string
          description: Error code
        message:
          type: string
          description: Human-readable error message
        timestamp:
          type: string
          format: date-time
          description: Error timestamp
        details:
          type: object
          description: Additional error details
```

### Rust API Implementation with Contract Validation
```rust
// Example: Rust API implementation with contract validation
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProcessRequest {
    #[validate(required, custom = "validate_data_type")]
    pub data_type: Option<String>,
    
    #[validate(required, url)]
    pub source: Option<String>,
    
    #[validate(required, custom = "validate_processor")]
    pub processor: Option<String>,
    
    #[validate(custom = "validate_output_format")]
    pub output_format: Option<String>,
    
    pub options: Option<ProcessingOptions>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProcessingOptions {
    #[validate(length(min = 1, max = 10))]
    pub delimiter: Option<String>,
    
    pub has_header: Option<bool>,
    
    #[validate(custom = "validate_compression")]
    pub compression: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResponse {
    pub job_id: Uuid,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub result_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

// Custom validators
fn validate_data_type(data_type: &str) -> Result<(), validator::ValidationError> {
    match data_type {
        "csv" | "json" | "parquet" | "avro" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_data_type")),
    }
}

fn validate_processor(processor: &str) -> Result<(), validator::ValidationError> {
    match processor {
        "rust-processor" | "python-processor" | "custom-processor" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_processor")),
    }
}

fn validate_output_format(format: &str) -> Result<(), validator::ValidationError> {
    match format {
        "json" | "csv" | "parquet" | "avro" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_output_format")),
    }
}

fn validate_compression(compression: &str) -> Result<(), validator::ValidationError> {
    match compression {
        "none" | "gzip" | "lz4" | "zstd" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_compression")),
    }
}

// API endpoint implementation
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data/process", post(process_data))
        .layer(ValidationLayer::new());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn process_data(
    ValidatedRequest(request): ValidatedRequest<ProcessRequest>,
) -> Result<Json<ProcessResponse>, StatusCode> {
    // Validate request against contract
    if let Err(validation_errors) = request.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Process the request
    let job_id = Uuid::new_v4();
    let response = ProcessResponse {
        job_id,
        status: JobStatus::Queued,
        created_at: Utc::now(),
        estimated_completion: Some(Utc::now() + chrono::Duration::minutes(5)),
        result_url: None,
    };
    
    Ok(Json(response))
}
```

### API Testing Strategy

#### Contract Testing
```rust
// Example: Contract testing with Rust
#[cfg(test)]
mod contract_tests {
    use super::*;
    use serde_json::json;
    use reqwest::Client;
    use tokio;

    #[tokio::test]
    async fn test_process_data_contract() {
        let client = Client::new();
        let request_body = json!({
            "data_type": "csv",
            "source": "https://example.com/data.csv",
            "processor": "rust-processor",
            "output_format": "json",
            "options": {
                "delimiter": ",",
                "has_header": true,
                "compression": "none"
            }
        });

        let response = client
            .post("http://localhost:3000/data/process")
            .json(&request_body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        
        let response_body: ProcessResponse = response.json().await.unwrap();
        
        // Validate response contract
        assert!(!response_body.job_id.to_string().is_empty());
        assert!(matches!(response_body.status, JobStatus::Queued));
        assert!(response_body.created_at <= Utc::now());
    }

    #[tokio::test]
    async fn test_invalid_request_contract() {
        let client = Client::new();
        let invalid_request = json!({
            "data_type": "invalid_type",
            "source": "not-a-url",
            "processor": "invalid-processor"
        });

        let response = client
            .post("http://localhost:3000/data/process")
            .json(&invalid_request)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 400);
        
        let error_response: ErrorResponse = response.json().await.unwrap();
        assert_eq!(error_response.error, "validation_error");
    }
}
```

#### Playwright API Testing
```typescript
// Example: Playwright API testing
import { test, expect } from '@playwright/test';

test.describe('API Contract Testing', () => {
  test('should process data according to contract', async ({ request }) => {
    const response = await request.post('/api/v1/data/process', {
      data: {
        data_type: 'csv',
        source: 'https://example.com/data.csv',
        processor: 'rust-processor',
        output_format: 'json',
        options: {
          delimiter: ',',
          has_header: true,
          compression: 'none'
        }
      }
    });

    expect(response.status()).toBe(200);
    
    const responseBody = await response.json();
    
    // Validate response contract
    expect(responseBody).toHaveProperty('job_id');
    expect(responseBody).toHaveProperty('status');
    expect(responseBody).toHaveProperty('created_at');
    expect(responseBody).toHaveProperty('estimated_completion');
    
    // Validate data types
    expect(typeof responseBody.job_id).toBe('string');
    expect(['queued', 'processing', 'completed', 'failed']).toContain(responseBody.status);
    expect(new Date(responseBody.created_at)).toBeInstanceOf(Date);
  });

  test('should reject invalid requests', async ({ request }) => {
    const response = await request.post('/api/v1/data/process', {
      data: {
        data_type: 'invalid_type',
        source: 'not-a-url',
        processor: 'invalid-processor'
      }
    });

    expect(response.status()).toBe(400);
    
    const errorBody = await response.json();
    expect(errorBody).toHaveProperty('error');
    expect(errorBody).toHaveProperty('message');
    expect(errorBody).toHaveProperty('timestamp');
  });

  test('should handle rate limiting', async ({ request }) => {
    const requests = Array(100).fill(null).map(() => 
      request.post('/api/v1/data/process', {
        data: {
          data_type: 'csv',
          source: 'https://example.com/data.csv',
          processor: 'rust-processor'
        }
      })
    );

    const responses = await Promise.all(requests);
    const rateLimitedResponses = responses.filter(r => r.status() === 429);
    
    expect(rateLimitedResponses.length).toBeGreaterThan(0);
  });
});
```

#### Performance Testing
```typescript
// Example: API performance testing
test.describe('API Performance Testing', () => {
  test('should handle concurrent requests efficiently', async ({ request }) => {
    const startTime = Date.now();
    
    const requests = Array(50).fill(null).map(() => 
      request.post('/api/v1/data/process', {
        data: {
          data_type: 'csv',
          source: 'https://example.com/data.csv',
          processor: 'rust-processor'
        }
      })
    );

    const responses = await Promise.all(requests);
    const endTime = Date.now();
    
    // All requests should succeed
    responses.forEach(response => {
      expect(response.status()).toBe(200);
    });
    
    // Should complete within 5 seconds
    expect(endTime - startTime).toBeLessThan(5000);
  });

  test('should process large datasets efficiently', async ({ request }) => {
    const startTime = Date.now();
    
    const response = await request.post('/api/v1/data/process', {
      data: {
        data_type: 'csv',
        source: 'https://example.com/large-dataset.csv',
        processor: 'rust-processor',
        options: {
          compression: 'lz4'
        }
      }
    });

    const endTime = Date.now();
    
    expect(response.status()).toBe(200);
    expect(endTime - startTime).toBeLessThan(10000); // 10 seconds max
  });
});
```

### Contract Validation and Enforcement

#### Runtime Validation
```rust
// Example: Runtime contract validation middleware
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};

pub async fn validate_contract(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract and validate request body
    let (parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Validate against OpenAPI schema
    if let Err(_) = validate_against_schema(&body_bytes, &parts.uri.path()).await {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let request = Request::from_parts(parts, axum::body::Body::from(body_bytes));
    let response = next.run(request).await;
    
    // Validate response against contract
    validate_response_contract(&response).await?;
    
    Ok(response)
}

async fn validate_against_schema(
    body: &[u8],
    path: &str,
) -> Result<(), ValidationError> {
    // Implementation of OpenAPI schema validation
    // This would use a library like jsonschema or similar
    Ok(())
}

async fn validate_response_contract(
    response: &Response,
) -> Result<(), StatusCode> {
    // Validate response structure against contract
    Ok(())
}
```

#### Contract Testing Tools
```yaml
# Example: Contract testing configuration
# .github/workflows/contract-tests.yml
name: Contract Tests
on: [push, pull_request]

jobs:
  contract-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Run contract tests
        run: |
          cargo test --package api-contracts
          
      - name: Validate OpenAPI schema
        run: |
          npx @apidevtools/swagger-cli validate api/openapi.yaml
          
      - name: Generate contract tests
        run: |
          npx @apidevtools/swagger-codegen-cli generate \
            -i api/openapi.yaml \
            -l typescript-fetch \
            -o tests/generated
```

## Quality Standards

### API Contract Requirements
- **Completeness**: All endpoints must be fully documented
- **Accuracy**: Contracts must match implementation exactly
- **Versioning**: Backward compatibility must be maintained
- **Validation**: Runtime contract validation must be enabled

### Testing Requirements
- **Contract compliance**: 100% of endpoints must have contract tests
- **Performance testing**: All endpoints must meet performance requirements
- **Error handling**: All error scenarios must be tested
- **Security testing**: All endpoints must pass security validation

## Tools and Technologies

### Contract Definition
- **OpenAPI 3.0**: REST API specification
- **AsyncAPI**: Event-driven API specification
- **GraphQL Schema**: GraphQL API specification
- **gRPC Proto**: gRPC service specification

### Validation and Testing
- **Rust**: `validator` crate for request validation
- **Playwright**: End-to-end API testing
- **Postman/Newman**: API testing and validation
- **Dredd**: API contract testing
- **Swagger Codegen**: Generate test clients

### Documentation
- **Swagger UI**: Interactive API documentation
- **ReDoc**: Alternative API documentation
- **Postman Collections**: API testing collections
- **OpenAPI Generator**: Generate client SDKs

## Metrics and Monitoring

### Contract Quality Metrics
- **Contract coverage**: Percentage of endpoints with contracts
- **Contract accuracy**: Percentage of contracts matching implementation
- **Validation success rate**: Percentage of valid requests
- **Documentation completeness**: Percentage of documented endpoints

### API Performance Metrics
- **Response time**: Average and P95 response times
- **Throughput**: Requests per second
- **Error rate**: Percentage of failed requests
- **Availability**: API uptime percentage

## Review and Updates
This ADR will be reviewed monthly to ensure API contract and testing strategy remains effective and aligned with client needs. Updates will be made based on:
- Client feedback on API quality
- New API standards and best practices
- Tool and technology updates
- Team experience and insights

## Related ADRs
- ADR-001: Test-Driven Development (TDD) First Approach
- ADR-002: Testing Pyramid Strategy
- ADR-003: Playwright Testing for Demos
- ADR-005: Performance Testing Strategy
