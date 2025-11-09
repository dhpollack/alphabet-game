help:
  just -l -u

dev:
  npx wrangler dev

deploy:
  npx wrangler deploy

d1-local-populate:
  npx wrangler d1 execute alphabet-game-stg --local --file=./sql/schema.sql

d1-local-query *args:
  npx wrangler d1 execute alphabet-game-stg --local --command="{{ args }}"

d1-remote-populate:
  npx wrangler d1 execute alphabet-game-stg --remote --file=./sql/schema.sql

d1-remote-query *args:
  npx wrangler d1 execute alphabet-game-stg --remote --command="{{ args }}"

