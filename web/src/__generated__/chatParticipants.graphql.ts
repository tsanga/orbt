/**
 * @generated SignedSource<<fab8cd83b0076e02cdb5ebe15b2bc73f>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { Fragment, ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type chatParticipants$data = {
  readonly members: ReadonlyArray<{
    readonly color: {
      readonly hex: string;
    };
    readonly user: {
      readonly name: string;
    };
  }>;
  readonly " $fragmentType": "chatParticipants";
};
export type chatParticipants$key = {
  readonly " $data"?: chatParticipants$data;
  readonly " $fragmentSpreads": FragmentRefs<"chatParticipants">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "chatParticipants",
  "selections": [
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

(node as any).hash = "30b6b73aa532d8cc6ed39a4135a84a9a";

export default node;
