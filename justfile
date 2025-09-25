default:
    just --list

lint:
    cargo clippy --all-features -- -D warnings

test:
    cargo nextest run --workspace --all-features --no-tests warn && \
      cargo test --doc --workspace

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

audit:
    cargo audit

check: fmt-check audit lint test readme-check

# -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
# Housekeeping

bump-master *args="":
    cargo workspaces version \
      --no-git-push \
      --no-individual-tags {{ args }}

bump-in-branch *args="":
    cargo workspaces version \
      --allow-branch $(git rev-parse --abbrev-ref HEAD) \
      --no-git-push \
      --no-global-tag \
      --no-individual-tags {{ args }}

# -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
# Release

readme:
    cargo rdme --force

readme-check dir=".":
    cd {{ dir }} && \
      cargo rdme --check
