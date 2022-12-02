/**
 * @generated SignedSource<<76423ab227432d807ccb214d44e1e433>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, GraphQLSubscription } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type watchViewRoomSubscription$variables = {
  id: string;
};
export type watchViewRoomSubscription$data = {
  readonly room: {
    readonly id: string;
    readonly " $fragmentSpreads": FragmentRefs<"chatBoxMessages" | "chatParticipants">;
  };
};
export type watchViewRoomSubscription = {
  response: watchViewRoomSubscription$data;
  variables: watchViewRoomSubscription$variables;
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
    "name": "id",
    "variableName": "id"
  }
],
v2 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "watchViewRoomSubscription",
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
    "type": "Subscription",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "watchViewRoomSubscription",
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
                  {
                    "alias": null,
                    "args": null,
                    "kind": "ScalarField",
                    "name": "name",
                    "storageKey": null
                  },
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
    "cacheID": "93d9bfda02ecbfa61705d5144c174217",
    "id": null,
    "metadata": {},
    "name": "watchViewRoomSubscription",
    "operationKind": "subscription",
    "text": "subscription watchViewRoomSubscription(\n  $id: Id!\n) {\n  room(id: $id) {\n    id\n    ...chatParticipants\n    ...chatBoxMessages\n  }\n}\n\nfragment chatBoxMessages on Room {\n  messages {\n    id\n    msg\n    author\n    time\n  }\n}\n\nfragment chatParticipants on Room {\n  members {\n    color {\n      hex\n    }\n    user {\n      name\n      id\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "6ef28afae77091bb1b44d5fdbabe2e1f";

export default node;
