/**
 * @generated SignedSource<<7f7ffc82b318306b2c9026d30e42ac49>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from 'relay-runtime';
export type chatBoxInputSendMessageMutation$variables = {
  msg: string;
  room: string;
};
export type chatBoxInputSendMessageMutation$data = {
  readonly sendChatMessage: {
    readonly id: string;
  };
};
export type chatBoxInputSendMessageMutation = {
  response: chatBoxInputSendMessageMutation$data;
  variables: chatBoxInputSendMessageMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = {
  "defaultValue": null,
  "kind": "LocalArgument",
  "name": "msg"
},
v1 = {
  "defaultValue": null,
  "kind": "LocalArgument",
  "name": "room"
},
v2 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Variable",
        "name": "msg",
        "variableName": "msg"
      },
      {
        "kind": "Variable",
        "name": "room",
        "variableName": "room"
      }
    ],
    "concreteType": "Room",
    "kind": "LinkedField",
    "name": "sendChatMessage",
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
    "argumentDefinitions": [
      (v0/*: any*/),
      (v1/*: any*/)
    ],
    "kind": "Fragment",
    "metadata": null,
    "name": "chatBoxInputSendMessageMutation",
    "selections": (v2/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [
      (v1/*: any*/),
      (v0/*: any*/)
    ],
    "kind": "Operation",
    "name": "chatBoxInputSendMessageMutation",
    "selections": (v2/*: any*/)
  },
  "params": {
    "cacheID": "eb6dc31f6b5aef0973ef34496a8272f4",
    "id": null,
    "metadata": {},
    "name": "chatBoxInputSendMessageMutation",
    "operationKind": "mutation",
    "text": "mutation chatBoxInputSendMessageMutation(\n  $room: Id!\n  $msg: String!\n) {\n  sendChatMessage(room: $room, msg: $msg) {\n    id\n  }\n}\n"
  }
};
})();

(node as any).hash = "63f3c663b3700eb1577ffe3926ce84c7";

export default node;
