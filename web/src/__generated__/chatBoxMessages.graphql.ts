/**
 * @generated SignedSource<<ae0b93183c5f7eae1998a3b4f7eeb826>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { Fragment, ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type chatBoxMessages$data = {
  readonly id: string;
  readonly members: ReadonlyArray<{
    readonly user: {
      readonly id: string;
      readonly name: string;
    };
  }>;
  readonly messages: ReadonlyArray<{
    readonly author: string;
    readonly id: string;
    readonly msg: string;
    readonly time: any;
  }>;
  readonly " $fragmentType": "chatBoxMessages";
};
export type chatBoxMessages$key = {
  readonly " $data"?: chatBoxMessages$data;
  readonly " $fragmentSpreads": FragmentRefs<"chatBoxMessages">;
};

const node: ReaderFragment = (function(){
var v0 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
};
return {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "chatBoxMessages",
  "selections": [
    (v0/*: any*/),
    {
      "alias": null,
      "args": null,
      "concreteType": "RoomChatMsg",
      "kind": "LinkedField",
      "name": "messages",
      "plural": true,
      "selections": [
        (v0/*: any*/),
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
    },
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
          "concreteType": "User",
          "kind": "LinkedField",
          "name": "user",
          "plural": false,
          "selections": [
            (v0/*: any*/),
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
      ],
      "storageKey": null
    }
  ],
  "type": "Room",
  "abstractKey": null
};
})();

(node as any).hash = "69ef278a2b60fbbfaf88efb58bf9640c";

export default node;
