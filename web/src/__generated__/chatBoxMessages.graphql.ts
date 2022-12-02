/**
 * @generated SignedSource<<79e443d65d02e86eb2cf4b9fa643ab46>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { Fragment, ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type chatBoxMessages$data = {
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

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "chatBoxMessages",
  "selections": [
    {
      "alias": null,
      "args": null,
      "concreteType": "RoomChatMsg",
      "kind": "LinkedField",
      "name": "messages",
      "plural": true,
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
  "type": "Room",
  "abstractKey": null
};

(node as any).hash = "fe8f45632942de38572668b85a237e20";

export default node;
