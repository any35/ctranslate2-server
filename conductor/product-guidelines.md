# Product Guidelines - ctranslate2-server

## Performance & Resource Management
- **Performance First:** Prioritize execution speed and memory efficiency in all architectural decisions.
- **Concurrency:** Maximize hardware utilization by processing as many concurrent requests as VRAM/RAM allows.
- **Efficient Logging:** Log performance metrics (latency, throughput, GPU utilization) and errors without introducing significant overhead.

## API & Interaction
- **OpenAI Compatibility:** Strictly adhere to the OpenAI API response formats, including error structures, to ensure seamless integration with existing clients.
- **Standard HTTP Protocols:** Use appropriate HTTP status codes (4xx/5xx) while providing OpenAI-formatted JSON error bodies.
- **Self-Documentation:** Provide an interactive OpenAPI/Swagger UI for easy discovery and testing of the endpoints.

## Observability & Maintenance
- **Detailed Tracking:** Maintain comprehensive logs for requests/responses (configurable level) and detailed error tracking.
- **Code Quality:** Use Rust's type system to enforce correctness and document the internal codebase extensively using doc comments (`///`).
- **Orchestration Friendly:** Implement health check endpoints to facilitate deployment in containerized environments like Kubernetes.

## Coding Philosophy
- **Rust Idioms:** Follow established Rust design patterns for safety and performance.
- **Hardware Optimization:** Explicitly handle CPU and GPU runtime differences to ensure optimized execution on both.
