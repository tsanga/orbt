/**
 * @generated SignedSource<<73d7b4d6ce865ec8bbc61a5e678fbd0f>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type chatParticipantInviteMutation$variables = {
  room: string;
};
export type chatParticipantInviteMutation$data = {
  readonly createRoomInvite: {
    readonly token: string | null;
  };
};
export type chatParticipantInviteMutation = {
  response: chatParticipantInviteMutation$data;
  variables: chatParticipantInviteMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "room"
  }
],
v1 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Variable",
        "name": "room",
        "variableName": "room"
      }
    ],
    "concreteType": "Token",
    "kind": "LinkedField",
    "name": "createRoomInvite",
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
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "chatParticipantInviteMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "chatParticipantInviteMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "e52386d13aa88e5788588388596e8c8a",
    "id": null,
    "metadata": {},
    "name": "chatParticipantInviteMutation",
    "operationKind": "mutation",
    "text": "mutation chatParticipantInviteMutation(\n  $room: Id!\n) {\n  createRoomInvite(room: $room) {\n    token\n  }\n}\n"
  }
};
})();

(node as any).hash = "19f709009d20f5cb7ae6acdc95be00f5";

export default node;
