/**
 * @generated SignedSource<<ed6bb20f84da6df300e41080b7396072>>
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
      readonly id: string;
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
            },
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
      ],
      "storageKey": null
    }
  ],
  "type": "Room",
  "abstractKey": null
};

(node as any).hash = "0976b20272d1feffa3a56639464d9404";

export default node;
