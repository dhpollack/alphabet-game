help:
  just -l -u

dev: sqlx-prepare
  npx wrangler dev --ip 0.0.0.0

deploy: sqlx-prepare
  npx wrangler deploy

sqlx-prepare:
  cargo sqlx prepare -- --features ssr

d1-migration-create *args:
  npx wrangler d1 migrations create alphabet-game-stg "{{ args }}"

d1-local-query *args:
  npx wrangler d1 execute alphabet-game-stg --local --command="{{ args }}"

d1-local-migration-apply:
  npx wrangler d1 migrations apply alphabet-game-stg

d1-remote-query *args:
  npx wrangler d1 execute alphabet-game-stg --remote --command="{{ args }}"

d1-remote-migration-apply:
  npx wrangler d1 migrations apply --remote alphabet-game-stg

_claude *args:
    claude {{ args }}

claude *args:
    just -E .env-claude _claude {{ args }}
