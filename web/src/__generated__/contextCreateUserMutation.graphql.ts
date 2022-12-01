/**
 * @generated SignedSource<<7f383d3c4e0dff6daa5a3dd2efa921c6>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type contextCreateUserMutation$variables = {
  name?: string | null;
};
export type contextCreateUserMutation$data = {
  readonly createUser: {
    readonly id: string;
    readonly name: string;
    readonly token: {
      readonly token: string | null;
    };
  };
};
export type contextCreateUserMutation = {
  response: contextCreateUserMutation$data;
  variables: contextCreateUserMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "name"
  }
],
v1 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Variable",
        "name": "name",
        "variableName": "name"
      }
    ],
    "concreteType": "User",
    "kind": "LinkedField",
    "name": "createUser",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "id",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "name",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "concreteType": "Token",
        "kind": "LinkedField",
        "name": "token",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "token",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "contextCreateUserMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "contextCreateUserMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "7d8373370fb403fc92a58e27f9ac5d5d",
    "id": null,
    "metadata": {},
    "name": "contextCreateUserMutation",
    "operationKind": "mutation",
    "text": "mutation contextCreateUserMutation(\n  $name: String\n) {\n  createUser(name: $name) {\n    id\n    name\n    token {\n      token\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "06852e186fc697b3b2ac206ecf0a78ed";

export default node;
