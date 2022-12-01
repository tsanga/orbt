/**
 * @generated SignedSource<<c9249819189b72af0cbdecd6fd79fc5a>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type useAuthCreateUserMutation$variables = {
  name?: string | null;
};
export type useAuthCreateUserMutation$data = {
  readonly createUser: {
    readonly id: any;
    readonly name: string;
    readonly token: {
      readonly token: string | null;
    };
  };
};
export type useAuthCreateUserMutation = {
  response: useAuthCreateUserMutation$data;
  variables: useAuthCreateUserMutation$variables;
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
    "name": "useAuthCreateUserMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "useAuthCreateUserMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "2ff4ad163d21ec904e4df36ef198964e",
    "id": null,
    "metadata": {},
    "name": "useAuthCreateUserMutation",
    "operationKind": "mutation",
    "text": "mutation useAuthCreateUserMutation(\n  $name: String\n) {\n  createUser(name: $name) {\n    id\n    name\n    token {\n      token\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "f0f2ccc9d55d82b4244eeecea9367913";

export default node;
