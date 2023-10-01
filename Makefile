.PHONY: get-test-id
get-test-id:
	curl -s localhost:3000/pessoas | jq -r ".[0].id"

.PHONY: get-person-by-id
get-person-by-id:
	curl -s localhost:3000/pessoas/018aed6f-3cd9-7355-b870-4efc6a9ac407 | jq
