# Contributing Guidelines

Thank you for contributing to Mortalbook! This guide outlines how to contribute effectively.

## Code of Conduct

- Treat all contributors with respect
- Provide constructive feedback
- Focus on the code, not the person
- Help others learn and improve

## Getting Started

1. **Fork the Repository**

   ```bash
   git clone https://github.com/your-username/mortalbook.git
   ```

2. **Create a Feature Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Follow the Development Setup**
   See [DEVELOPMENT.md](DEVELOPMENT.md) for local setup instructions.

## Development Workflow

### Before Starting

- Check [Issues](https://github.com/yourorg/mortalbook/issues) for existing work
- Create an issue for new features or bugs
- Discuss design decisions before coding

### Commit Messages

Use clear, descriptive commit messages:

```
feat(memorial-service): add memorial search functionality

- Implement full-text search in memorial repository
- Add search endpoint to HTTP handlers
- Include search field in API documentation

Closes #123
```

**Format:**

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting, semicolons, etc.)
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Test additions or changes
- `chore`: Build, CI, dependencies

**Scope:** Service name or module (memorial-service, admin-service, frontend, etc.)

**Subject:** Imperative mood, first word capitalized, no period

**Body:** Explain what and why, not how

**Footer:** Reference issues: `Closes #123` or `Fixes #456`

### Pull Requests

1. **Create PR Description**
   - Clear title with issue reference
   - Description of changes
   - Screenshots for UI changes
   - Testing instructions

2. **Template Example**

   ```markdown
   ## Description

   This PR adds search functionality to the memorial listing page.

   Closes #123

   ## Changes

   - Added full-text search repository method
   - Created search endpoint in HTTP handlers
   - Updated API documentation

   ## Testing

   1. Run `go test ./...`
   2. Start memorial service: `go run cmd/main.go`
   3. Test search: `curl "http://localhost:5001/api/v1/memorials?search=john"`

   ## Screenshots

   [If applicable]
   ```

3. **Review Process**
   - Maintainers will review within 2 days
   - Address feedback constructively
   - Ensure CI passes before merge

## Code Quality Standards

### Naming Conventions

**Go:**

```go
// Constants
const MaxPageSize = 100

// Functions
func CreateMemorial(ctx context.Context, memorial *Memorial) error

// Private functions
func validateMemorial(memorial *Memorial) error

// Interfaces
type MemorialRepository interface {
    GetByID(ctx context.Context, id string) (*Memorial, error)
}
```

**Python:**

```python
# Constants
MAX_PAGE_SIZE = 100

# Functions
def create_memorial(db: Session, memorial: MemorialCreate) -> Memorial:
    pass

# Private functions
def _validate_memorial(memorial: Memorial) -> bool:
    pass

# Classes
class MemorialRepository:
    def get_by_id(self, memorial_id: str) -> Optional[Memorial]:
        pass
```

**JavaScript/Vue:**

```javascript
// Constants
const MAX_PAGE_SIZE = 100;

// Functions
function createMemorial(memorial) {}

// Private functions
function _validateMemorial(memorial) {}

// Classes
class MemorialService {
  getById(id) {}
}
```

### Error Handling

**Go:**

```go
// Always return errors
func GetMemorial(ctx context.Context, id string) (*Memorial, error) {
    memorial, err := repo.GetByID(ctx, id)
    if err != nil {
        // Log error at service layer
        logger.Error("failed to get memorial", zap.Error(err), zap.String("id", id))
        return nil, fmt.Errorf("failed to get memorial: %w", err)
    }
    return memorial, nil
}
```

**Python:**

```python
# Use try-except for I/O operations
try:
    memorial = db.query(Memorial).filter(Memorial.id == id).first()
    if not memorial:
        raise HTTPException(status_code=404, detail="Memorial not found")
except SQLAlchemyError as e:
    logger.error(f"Database error: {e}")
    raise HTTPException(status_code=500, detail="Internal server error")
```

**JavaScript:**

```javascript
// Handle promises properly
async function getMemorial(id) {
  try {
    const response = await fetch(`/api/memorials/${id}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return await response.json();
  } catch (error) {
    console.error("Failed to get memorial:", error);
    throw error;
  }
}
```

### Testing Requirements

**All changes must include tests:**

**Go:**

```go
func TestMemorialRepository_GetByID_Success(t *testing.T) {
    repo := setupTestRepository(t)
    defer repo.Close()

    memorial, err := repo.GetByID(context.Background(), "test-id")

    assert.NoError(t, err)
    assert.NotNil(t, memorial)
    assert.Equal(t, "test-id", memorial.ID)
}

func TestMemorialRepository_GetByID_NotFound(t *testing.T) {
    repo := setupTestRepository(t)
    defer repo.Close()

    _, err := repo.GetByID(context.Background(), "nonexistent")

    assert.Error(t, err)
}
```

**Python:**

```python
def test_create_memorial_success(client, db):
    payload = {
        "name": "Test Person",
        "date_of_birth": "1950-01-01",
        "date_of_death": "2024-01-10"
    }
    response = client.post("/memorials", json=payload)
    assert response.status_code == 201
    assert response.json()["name"] == "Test Person"

def test_create_memorial_invalid_data(client):
    payload = {"name": "Test"}  # Missing required fields
    response = client.post("/memorials", json=payload)
    assert response.status_code == 400
```

**TypeScript:**

```typescript
describe("MemorialCard", () => {
  it("renders memorial information", () => {
    const memorial = {
      id: "1",
      name: "John Doe",
      dateOfBirth: "1950-01-01",
      dateOfDeath: "2024-01-10"
    };

    const { getByText } = render(<MemorialCard memorial={memorial} />);
    expect(getByText("John Doe")).toBeInTheDocument();
  });
});
```

### Documentation

**Docstrings/Comments:**

**Go:**

```go
// MemorialService handles memorial business logic.
type MemorialService struct {
    repo repository.MemorialRepository
}

// CreateMemorial creates a new memorial and publishes event.
// Returns error if validation fails or database operation fails.
func (s *MemorialService) CreateMemorial(ctx context.Context, memorial *Memorial) error {
    // Implementation
}
```

**Python:**

```python
class MemorialService:
    """Service for memorial operations."""

    def create_memorial(self, db: Session, memorial: MemorialCreate) -> Memorial:
        """
        Create a new memorial.

        Args:
            db: Database session
            memorial: Memorial data to create

        Returns:
            Created memorial with ID

        Raises:
            ValueError: If data is invalid
            DatabaseError: If operation fails
        """
```

**Vue:**

```vue
<!-- MemorialCard.vue -->
<template>
  <!-- Display memorial basic information -->
  <div class="memorial-card">
    <!-- ... -->
  </div>
</template>

<script setup>
/**
 * MemorialCard component
 * @param {Object} memorial - Memorial data object with id, name, dates
 * @param {Function} onDelete - Callback when delete button clicked
 */
import { defineProps, defineEmits } from "vue";

defineProps({
  memorial: {
    type: Object,
    required: true,
  },
});

const emit = defineEmits(["delete"]);
</script>
```

## Performance Checklist

Before submitting, ensure your code:

- [ ] Has no obvious performance issues
- [ ] Uses efficient database queries (indexes, pagination)
- [ ] Implements appropriate caching
- [ ] Doesn't introduce N+1 queries
- [ ] Uses connection pooling
- [ ] Handles large datasets with pagination
- [ ] Has reasonable complexity (O(n log n) or better typically)

**Performance review:**

```bash
# Go: benchmark tests
go test -bench=. ./...

# Database: check query plans
EXPLAIN ANALYZE SELECT ...;

# Frontend: lighthouse audit
npm run build && npm run analyze
```

## Accessibility Standards

**WCAG 2.1 AA Compliance:**

- [ ] Semantic HTML (use `<button>`, `<form>`, etc.)
- [ ] ARIA labels on interactive elements
- [ ] Color contrast 4.5:1 for text
- [ ] Keyboard navigation support
- [ ] Alt text on images
- [ ] Focus indicators visible
- [ ] Error messages clear and associated with inputs

```vue
<!-- Good example -->
<button @click="deleteMemorial" aria-label="Delete memorial" class="btn-danger">
  Delete
</button>

<!-- Bad example -->
<div @click="deleteMemorial">X</div>
```

## Security Checklist

Before submitting security-related changes:

- [ ] No hardcoded credentials or secrets
- [ ] Input validation on all user inputs
- [ ] Proper error handling (don't leak sensitive info)
- [ ] SQL injection protection (use parameterized queries)
- [ ] CSRF token validation where needed
- [ ] Authentication required on protected endpoints
- [ ] Authorization checks on resources
- [ ] HTTPS/TLS used in production
- [ ] Secrets managed securely (environment variables, K8s secrets)

## Documentation Updates

Update documentation for:

- New features or API changes
- Breaking changes
- Configuration options
- New environment variables

Update these files:

- `docs/API.md` for API changes
- `docs/ARCHITECTURE.md` for design decisions
- `docs/DEVELOPMENT.md` for setup/workflow changes
- README.md for project overview changes
- Service-specific README.md files

## Review Guidelines for Reviewers

When reviewing PRs, check:

1. **Code Quality**
   - Follows project conventions
   - Clean, readable, maintainable
   - Proper error handling

2. **Testing**
   - Tests are included
   - Tests are meaningful
   - Coverage is adequate

3. **Documentation**
   - Code is documented
   - User-facing docs updated
   - Commit messages are clear

4. **Performance**
   - No performance regressions
   - Efficient algorithms
   - Appropriate caching

5. **Security**
   - No vulnerabilities introduced
   - Proper input validation
   - Secrets not exposed

## Reporting Issues

**Bug Report Template:**

```markdown
## Description

Brief description of the bug.

## Steps to Reproduce

1. Do this
2. Then this
3. Observe this

## Expected Behavior

What should happen?

## Actual Behavior

What actually happened?

## Environment

- OS: (Windows/macOS/Linux)
- Go/Python/Node version:
- Service: (memorial-service/admin-service/frontend/admin-panel)

## Logs/Screenshots

[Relevant logs or screenshots]
```

**Feature Request Template:**

```markdown
## Description

Brief description of the requested feature.

## Use Case

Why do you need this feature?

## Proposed Solution

How should it work?

## Alternatives

Are there other ways to solve this?
```

## Licensing

By contributing, you agree that your contributions will be licensed under the same license as the project.

## Questions?

- Open an issue for questions
- Ask in our discussion forum
- Contact maintainers

---

**Thank you for contributing to Mortalbook!** 🙏
