/**
 * @generated SignedSource<<b4eb8ddf08ad84a04a15b92589c57b5d>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type joinViewRoomMutation$variables = {};
export type joinViewRoomMutation$data = {
  readonly joinRoom: {
    readonly id: string;
  };
};
export type joinViewRoomMutation = {
  response: joinViewRoomMutation$data;
  variables: joinViewRoomMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "Room",
    "kind": "LinkedField",
    "name": "joinRoom",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "id",
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "joinViewRoomMutation",
    "selections": (v0/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "joinViewRoomMutation",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "8d2767dde10d212123899b43364a7aba",
    "id": null,
    "metadata": {},
    "name": "joinViewRoomMutation",
    "operationKind": "mutation",
    "text": "mutation joinViewRoomMutation {\n  joinRoom {\n    id\n  }\n}\n"
  }
};
})();

(node as any).hash = "7484831080fed660f46ba55b27dea061";

export default node;
