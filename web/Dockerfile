FROM node:slim

# make user
RUN useradd -m orbt
USER orbt

WORKDIR /app

# copy dep list first so we can cache em
COPY --chown=orbt:orbt yarn.lock .
COPY --chown=orbt:orbt package.json .
COPY --chown=orbt:orbt patches .

RUN yarn install

# copy the rest of the app
COPY --chown=orbt:orbt . .

ENTRYPOINT [ "yarn" ]
CMD [ "dev" ]
