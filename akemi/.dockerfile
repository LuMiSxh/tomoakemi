# BUILD
FROM node:18

WORKDIR .
COPY . .

RUN pnpm install && pnpm run lint && pnpm run format && pnpm run build

# RUN

CMD ["pnpm run preview"]
