FROM node:slim

RUN yarn global add patch-package

WORKDIR /app

# copy dep list first so we can cache em
COPY yarn.lock /app
COPY package.json /app
COPY patches /app
RUN yarn install

# copy the rest of the app
COPY . /app

# make user
RUN useradd -m orbt
RUN chown -R orbt:orbt /app
USER orbt

ENTRYPOINT [ "yarn" ]
CMD [ "dev" ]
