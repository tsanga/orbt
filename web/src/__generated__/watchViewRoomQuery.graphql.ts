/**
 * @generated SignedSource<<8eff80d711d7a6c81555f07ad743d469>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Query } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type watchViewRoomQuery$variables = {
  id: string;
};
export type watchViewRoomQuery$data = {
  readonly room: {
    readonly id: string;
    readonly name: string;
    readonly " $fragmentSpreads": FragmentRefs<"chatBoxMessages" | "chatParticipants">;
  } | null;
};
export type watchViewRoomQuery = {
  response: watchViewRoomQuery$data;
  variables: watchViewRoomQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "id"
  }
],
v1 = [
  {
    "kind": "Variable",
    "name": "room",
    "variableName": "id"
  }
],
v2 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
},
v3 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "name",
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "watchViewRoomQuery",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "Room",
        "kind": "LinkedField",
        "name": "room",
        "plural": false,
        "selections": [
          (v2/*: any*/),
          (v3/*: any*/),
          {
            "args": null,
            "kind": "FragmentSpread",
            "name": "chatParticipants"
          },
          {
            "args": null,
            "kind": "FragmentSpread",
            "name": "chatBoxMessages"
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "watchViewRoomQuery",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "Room",
        "kind": "LinkedField",
        "name": "room",
        "plural": false,
        "selections": [
          (v2/*: any*/),
          (v3/*: any*/),
          {
            "alias": null,
            "args": null,
            "concreteType": "RoomMember",
            "kind": "LinkedField",
            "name": "members",
            "plural": true,
            "selections": [
              {
                "alias": null,
                "args": null,
                "concreteType": "Color",
                "kind": "LinkedField",
                "name": "color",
                "plural": false,
                "selections": [
                  {
                    "alias": null,
                    "args": null,
                    "kind": "ScalarField",
                    "name": "hex",
                    "storageKey": null
                  }
                ],
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "concreteType": "User",
                "kind": "LinkedField",
                "name": "user",
                "plural": false,
                "selections": [
                  (v3/*: any*/),
                  (v2/*: any*/)
                ],
                "storageKey": null
              }
            ],
            "storageKey": null
          },
          {
            "alias": null,
            "args": null,
            "concreteType": "RoomChatMsg",
            "kind": "LinkedField",
            "name": "messages",
            "plural": true,
            "selections": [
              (v2/*: any*/),
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "msg",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "author",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "time",
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "72ab92ac864ea5c39bc171fbe2f72433",
    "id": null,
    "metadata": {},
    "name": "watchViewRoomQuery",
    "operationKind": "query",
    "text": "query watchViewRoomQuery(\n  $id: Id!\n) {\n  room(room: $id) {\n    id\n    name\n    ...chatParticipants\n    ...chatBoxMessages\n  }\n}\n\nfragment chatBoxMessages on Room {\n  messages {\n    id\n    msg\n    author\n    time\n  }\n}\n\nfragment chatParticipants on Room {\n  members {\n    color {\n      hex\n    }\n    user {\n      name\n      id\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "3a3357bc015d46265b489b8f4ce206d2";

export default node;
