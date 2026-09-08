# syntax=docker/dockerfile:1
# --- Build: install the whole workspace, build the web member ---
FROM node:20-alpine AS build
WORKDIR /app
# Root manifests first, then EVERY member's — a workspace install resolves
# the whole graph and fails outright when a declared member is missing.
COPY package.json pnpm-lock.yaml* pnpm-workspace.yaml* .npmrc* ./
COPY apps/web/package.json apps/web/
RUN corepack enable && if [ -f pnpm-lock.yaml ]; then pnpm install --frozen-lockfile; else pnpm install; fi
COPY . .
RUN corepack enable && pnpm --filter web build

# --- Runtime: the built workspace, run from apps/web ---
FROM node:20-alpine AS runtime
ENV NODE_ENV=production
WORKDIR /app
COPY --from=build --chown=1000:1000 /app ./
# Numeric USER — runAsNonRoot rejects a name it cannot verify from outside
# the image. `node` in this image is uid 1000.
USER 1000
WORKDIR /app/apps/web
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=20s \
  CMD node -e "fetch('http://127.0.0.1:3000/').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"
CMD ["npx", "next", "start", "-p", "3000"]
