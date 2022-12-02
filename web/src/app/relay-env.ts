import {
  Environment,
  Network,
  RecordSource,
  Store,
  RequestParameters,
  QueryResponseCache,
  Variables,
  GraphQLResponse,
  CacheConfig,
  EnvironmentConfig,
  Observable,
} from "relay-runtime";

import { Client, createClient } from "graphql-ws";

const IS_SERVER = typeof window === typeof undefined;
const CACHE_TTL = 5 * 1000; // 5 seconds, to resolve preloaded results

function fetchQuery(operation: any, variables: any, extendHeaders: any) {
  return fetch("http://localhost:8080/", {
    method: "POST",
    headers: {
      // Add authentication and other headers here
      Accept: "application/json",
      "content-type": "application/json",
      ...extendHeaders,
    },
    body: JSON.stringify({
      id: operation.id,
      query: operation.text, // GraphQL text from input
      variables,
    }),
    cache: "no-store",
  }).then((response) => {
    return response.json();
  });
}

export const responseCache: QueryResponseCache | null = IS_SERVER
  ? null
  : new QueryResponseCache({
      size: 100,
      ttl: CACHE_TTL,
    });

function createNetwork(extendHeaders: any) {
  const client = IS_SERVER
    ? null
    : createClient({
        url: "ws://localhost:8080",
        connectionParams: async () => {
          return extendHeaders;
        },
      });

  function fetch(operation: RequestParameters, variables: Variables) {
    return fetchQuery(operation, variables, extendHeaders);
  }

  function subscribe(
    operation: RequestParameters,
    variables: Variables
  ): Observable<any> {
    return Observable.create((sink) => {
      if (!operation.text) {
        return sink.error(new Error("Operation text cannot be empty"));
      }
      return client?.subscribe(
        {
          operationName: operation.name,
          query: operation.text,
          variables,
        },
        sink
      );
    });
  }

  return Network.create(fetch, subscribe);
}

// Export a singleton instance of Relay Environment configured with our network function:
export default function getEnvironment(
  extendConfig: Partial<EnvironmentConfig> = {},
  extendHeaders: any = {}
) {
  return new Environment({
    ...extendConfig,
    network: createNetwork(extendHeaders),
    store: new Store(RecordSource.create()),
  });
}
