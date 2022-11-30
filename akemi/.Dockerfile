# BUILD
FROM node:18

WORKDIR .
COPY . .

RUN npm install -g pnpm

RUN pnpm install pnpm run build

# RUN

CMD ["pnpm run preview"]
