/**
 * @generated SignedSource<<719f45f25b9386a7d73bc3c0fb98aa15>>
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
    readonly id: number;
    readonly name: string;
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
    "cacheID": "1c3d66c580bd9e5c9390b3de755a366d",
    "id": null,
    "metadata": {},
    "name": "useAuthCreateUserMutation",
    "operationKind": "mutation",
    "text": "mutation useAuthCreateUserMutation(\n  $name: String\n) {\n  createUser(name: $name) {\n    id\n    name\n  }\n}\n"
  }
};
})();

(node as any).hash = "a67d18b8bc51f17bf14975763d152884";

export default node;
