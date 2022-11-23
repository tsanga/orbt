import { Environment, Network, RecordSource, Store } from "relay-runtime";

function fetchQuery(operation: string, variables: any) {
  return fetch("http://localhost:8080/", {
    method: "POST",
    headers: {
      // Add authentication and other headers here
      "content-type": "application/json",
    },
    body: JSON.stringify({
      query: operation, // GraphQL text from input
      variables,
    }),
  }).then((response) => {
    return response.json();
  });
}

// Relay passes a "params" object with the query name and text. So we define a helper function
// to call our fetchGraphQL utility with params.text.
async function fetchRelay(params: any, variables: any) {
  console.log(
    `fetching query ${params.name} with ${JSON.stringify(variables)}`
  );
  return fetchQuery(params.text, variables);
}

// Export a singleton instance of Relay Environment configured with our network function:
export default new Environment({
  network: Network.create(fetchRelay),
  store: new Store(new RecordSource()),
});
