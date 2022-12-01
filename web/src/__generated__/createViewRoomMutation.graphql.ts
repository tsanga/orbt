/**
 * @generated SignedSource<<bcf9b43c62e9c9f02ec504ec72aab395>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type createViewRoomMutation$variables = {
  name?: string | null;
};
export type createViewRoomMutation$data = {
  readonly createRoom: {
    readonly id: string;
    readonly name: string;
  };
};
export type createViewRoomMutation = {
  response: createViewRoomMutation$data;
  variables: createViewRoomMutation$variables;
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
    "concreteType": "Room",
    "kind": "LinkedField",
    "name": "createRoom",
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
    "name": "createViewRoomMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "createViewRoomMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "543119b274ac38debfc16e80af52632b",
    "id": null,
    "metadata": {},
    "name": "createViewRoomMutation",
    "operationKind": "mutation",
    "text": "mutation createViewRoomMutation(\n  $name: String\n) {\n  createRoom(name: $name) {\n    id\n    name\n  }\n}\n"
  }
};
})();

(node as any).hash = "b4078b7734525483bfbabfb25d0f2a80";

export default node;
