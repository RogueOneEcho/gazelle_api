# Testing

## Running Tests

Run unit tests (integration tests are ignored by default):

```bash
cargo test
```

Run only library unit tests:

```bash
cargo test --lib
```

Run only integration tests (requires API credentials):

```bash
cargo test -- --ignored
```

Run all tests including integration tests:

```bash
cargo test -- --include-ignored
```

## Test Structure

Tests follow standard Rust conventions with `#[cfg(test)] mod tests` blocks inside each source file.

### Unit Tests

Unit tests use JSON fixtures and don't require API credentials:

| File                               | Tests                            |
|------------------------------------|----------------------------------|
| `src/schema/torrent_response.rs`   | TorrentResponse deserialization  |
| `src/schema/group_response.rs`     | GroupResponse deserialization    |
| `src/schema/user.rs`               | User deserialization             |
| `src/schema/torrent.rs`            | Torrent helper methods           |
| `src/client.rs`                    | JSON parsing and error handling  |
| `src/error.rs`                     | Error matching and serialization |
| `src/tests/rate_limiter_tests.rs`  | Rate limiter behavior            |

### Integration Tests

Integration tests are marked with `#[ignore]` by default as they make real API calls and require credentials in `config.yml`. Run them with `cargo test -- --ignored`.

| File                               | Tests                           |
|------------------------------------|---------------------------------|
| `src/actions/get_torrent.rs`       | Fetch single torrent            |
| `src/actions/get_torrent_group.rs` | Fetch torrent group             |
| `src/actions/get_user.rs`          | Fetch user profile              |
| `src/actions/download_torrent.rs`  | Download .torrent file          |
| `src/actions/upload_torrent.rs`    | Upload torrent (always ignored) |

## Fixtures

JSON fixtures in `src/tests/fixtures/` are based on real API responses with sanitized data:

| File                            | Description               |
|---------------------------------|---------------------------|
| `torrent_response_ops.json`     | OPS torrent endpoint      |
| `torrent_response_red.json`     | RED torrent endpoint      |
| `torrent_response_minimal.json` | Minimal fields            |
| `group_response_ops.json`       | OPS torrentgroup endpoint |
| `group_response_red.json`       | RED torrentgroup endpoint |
| `user_response_ops.json`        | OPS user endpoint         |
| `user_response_red.json`        | RED user endpoint         |
| `error_response_ops.json`       | OPS error format          |
| `error_response_red.json`       | RED error format          |

Fixtures capture key differences between OPS and RED:

| Field              | OPS                            | RED        |
|--------------------|--------------------------------|------------|
| BB code body       | `wikiBBcode` (ignored)         | `bbBody`   |
| `trumpable`        | Present                        | Present    |
| `lossyWebApproved` | Absent                         | Present    |
| `isNeutralleech`   | Absent                         | Present    |
| `isFreeload`       | Absent                         | Present    |
| `bbProfileText`    | Absent                         | Present    |
| Error response     | Has malformed `"response":[]`  | Clean JSON |

## Configuration

Integration tests require `config.yml` with API credentials. Copy the example and add your keys:

```bash
cp config.example.yml config.yml
```
