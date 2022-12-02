/**
 * @generated SignedSource<<d4c22779dfd41f3c2bbef85e07ce005b>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type joinViewRoomMutation$variables = {
  name: string;
};
export type joinViewRoomMutation$data = {
  readonly joinRoom: {
    readonly id: string;
  };
  readonly setUserName: {
    readonly id: string;
    readonly name: string;
    readonly token: {
      readonly token: string | null;
    };
  };
};
export type joinViewRoomMutation = {
  response: joinViewRoomMutation$data;
  variables: joinViewRoomMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "name"
  }
],
v1 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
},
v2 = [
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
    "name": "setUserName",
    "plural": false,
    "selections": [
      (v1/*: any*/),
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
  },
  {
    "alias": null,
    "args": null,
    "concreteType": "Room",
    "kind": "LinkedField",
    "name": "joinRoom",
    "plural": false,
    "selections": [
      (v1/*: any*/)
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "joinViewRoomMutation",
    "selections": (v2/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "joinViewRoomMutation",
    "selections": (v2/*: any*/)
  },
  "params": {
    "cacheID": "a8277a7c69036b897ead56ae61f0faee",
    "id": null,
    "metadata": {},
    "name": "joinViewRoomMutation",
    "operationKind": "mutation",
    "text": "mutation joinViewRoomMutation(\n  $name: String!\n) {\n  setUserName(name: $name) {\n    id\n    name\n    token {\n      token\n    }\n  }\n  joinRoom {\n    id\n  }\n}\n"
  }
};
})();

(node as any).hash = "f33f355bc356d6f36c65c7d049125b56";

export default node;
