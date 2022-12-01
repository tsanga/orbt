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
} from "relay-runtime";

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
  async function fetchResponse(
    params: RequestParameters,
    variables: Variables,
    cacheConfig: CacheConfig
  ) {
    const isQuery = params.operationKind === "query";
    const cacheKey = params.id ?? params.cacheID;
    const forceFetch = cacheConfig && cacheConfig.force;
    if (responseCache != null && isQuery && !forceFetch) {
      const fromCache = responseCache.get(cacheKey, variables);
      if (fromCache != null) {
        return Promise.resolve(fromCache);
      }
    }

    return fetchQuery(params, variables, extendHeaders);
  }
  const network = Network.create(fetchResponse);
  return network;
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
